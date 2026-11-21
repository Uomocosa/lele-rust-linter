use std::process;

use clap::Parser;
use lele_lint::CommonArgs;
use lele_lint::Config;
use lele_lint::ConfigSync;
use lele_lint::Linter;
use lele_lint::Project;
use lele_lint::RelaxedRule;
use lele_lint::RuleSnapshot;

#[derive(Parser)]
#[command(
    name = "lele_lint",
    version,
    about = "Enforce lele-syntax-rs conventions"
)]
struct Args {
    #[command(flatten)]
    common: CommonArgs,

    #[arg(long = "rules-md")]
    rules_md: bool,

    #[arg(long = "skill-md")]
    skill_md: bool,

    #[arg(long = "sync-methods")]
    sync_methods: bool,
}

fn main() {
    let args = Args::parse();

    let mut linter = Linter {
        error_format: args.common.error_format,
        ..Linter::default()
    };

    if args.common.rule_list || args.rules_md || args.skill_md || args.common.explain.is_some() {
        if let Some(codes) = &args.common.only {
            let unknown = linter.retain_codes(codes);
            if !unknown.is_empty() {
                eprintln!(
                    "lele_lint: no rule with code or name `{}`",
                    unknown.join("`, `")
                );
                process::exit(1);
            }
        }

        if args.common.rule_list {
            linter.print_rule_list();
        } else if args.rules_md {
            print!("{}", linter.rules_markdown());
        } else if let Some(code) = &args.common.explain {
            let Some(text) = linter.explain(code) else {
                eprintln!("lele_lint: no rule with code or name `{code}`");
                process::exit(1);
            };
            print!("{text}");
        } else {
            print!("{}", linter.skill_markdown());
        }
        return;
    }

    let mut project = match Project::try_from_start_dir_and_scan_folders(
        args.common.path.as_deref(),
        args.common.scan_folder.as_deref(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("lele_lint: {}", lele_lint::Error::with_causes(&e));
            process::exit(1);
        }
    };

    if args.common.sync.sync_config {
        let config_path = ConfigSync::config_path(args.common.config.as_deref(), &project);
        let outcome =
            match ConfigSync::synchronize(&linter.rules, &config_path, args.common.sync.check) {
                Ok(outcome) => outcome,
                Err(e) => {
                    eprintln!("lele_lint: {}", lele_lint::Error::with_causes(&e));
                    process::exit(1);
                }
            };
        if args.common.sync.check {
            let report = ConfigSync::render_sync_check(&outcome, "lele-lint");
            if report.is_empty() {
                return;
            }
            eprint!("{report}");
            process::exit(1);
        }
        return;
    }

    let config = match Config::resolve(args.common.config.as_deref(), &project) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "lele_lint: lele.toml: {}",
                lele_lint::Error::with_causes(&e)
            );
            process::exit(1);
        }
    };
    let (config, config_dir) = config;
    project.config_dir = config_dir;

    let snapshot: Vec<RuleSnapshot> = linter
        .rules
        .iter()
        .map(|rule| RuleSnapshot {
            name: rule.name().to_owned(),
            code: rule.code().to_owned(),
            namespace: rule.namespace().to_owned(),
            is_locked: rule.is_locked(),
        })
        .collect();

    if let Err(e) = project.apply_layout(&config) {
        eprintln!("lele_lint: {}", lele_lint::Error::with_causes(&e));
        process::exit(1);
    }

    let relaxed = match linter.apply_rule_settings(&config) {
        Ok(relaxed) => relaxed,
        Err(e) => {
            eprintln!("lele_lint: {}", lele_lint::Error::with_causes(&e));
            process::exit(1);
        }
    };

    if args.common.config_report {
        let mut snapshot = snapshot;
        if let Some(codes) = &args.common.only {
            let unknown: Vec<String> = codes
                .iter()
                .filter(|code| {
                    !snapshot.iter().any(|entry| {
                        entry.code.eq_ignore_ascii_case(code) || entry.name == code.as_str()
                    })
                })
                .cloned()
                .collect();
            if !unknown.is_empty() {
                eprintln!(
                    "lele_lint: no rule with code or name `{}`",
                    unknown.join("`, `")
                );
                process::exit(1);
            }
            snapshot.retain(|entry| {
                codes.iter().any(|code| {
                    entry.code.eq_ignore_ascii_case(code) || entry.name == code.as_str()
                })
            });
        }
        print!("{}", ConfigSync::render_config_report(&snapshot, &relaxed));
        return;
    }

    if let Some(codes) = &args.common.only {
        let unknown = linter.retain_codes(codes);
        if !unknown.is_empty() {
            eprintln!(
                "lele_lint: no rule with code or name `{}`",
                unknown.join("`, `")
            );
            process::exit(1);
        }
    }
    let relaxed: Vec<RelaxedRule> = relaxed
        .into_iter()
        .filter(|entry| entry.applies_to(&project))
        .collect();
    eprint!("{}", Linter::render_relaxed_banner(&relaxed));

    if !project.parse_failures.is_empty() {
        for failure in &project.parse_failures {
            eprintln!(
                "lele_lint: could not parse {}: {}",
                failure.path.display(),
                failure.cause
            );
        }
        process::exit(1);
    }

    if args.sync_methods {
        if let Err(e) = project.sync_methods() {
            eprintln!("lele_lint: {}", lele_lint::Error::with_causes(&e));
            process::exit(1);
        }
        return;
    }

    process::exit(linter.run(&project));
}
