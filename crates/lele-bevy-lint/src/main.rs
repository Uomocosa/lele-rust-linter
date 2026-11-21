use std::process;

use clap::Parser;
use clap::ValueEnum;
use lele_bevy_lint::BevyLinter;
use lele_bevy_lint::scan;
use lele_lint::CommonArgs;
use lele_lint::ConfigSync;
use lele_lint::Linter;
use lele_lint::Project;
use lele_lint::RuleSnapshot;

#[derive(Clone, Copy, ValueEnum)]
enum Emit {
    RulesMd,
    SkillMd,
}

#[derive(Parser)]
#[command(
    name = "lele_bevy_lint",
    version,
    about = "Enforce Bevy-specific lele-syntax-rs conventions"
)]
struct Args {
    #[command(flatten)]
    common: CommonArgs,

    #[arg(long, value_enum, value_name = "KIND")]
    emit: Option<Emit>,

    #[arg(long = "ui-inventory")]
    ui_inventory: bool,
}

fn main() {
    let args = Args::parse();

    let mut linter = BevyLinter::from(args.common.error_format);

    if args.common.rule_list || args.emit.is_some() || args.common.explain.is_some() {
        if let Some(codes) = &args.common.only {
            let unknown = linter.retain_codes(codes);
            if !unknown.is_empty() {
                eprintln!(
                    "lele_bevy_lint: no rule with code or name `{}`",
                    unknown.join("`, `")
                );
                process::exit(1);
            }
        }

        if args.common.rule_list {
            linter.print_rule_list();
            return;
        }

        match args.emit {
            Some(Emit::RulesMd) => {
                print!("{}", linter.rules_markdown());
                return;
            }
            Some(Emit::SkillMd) => {
                print!("{}", linter.skill_markdown());
                return;
            }
            None => {}
        }

        if let Some(code) = &args.common.explain {
            let Some(text) = linter.explain(code) else {
                eprintln!("lele_bevy_lint: no rule with code or name `{code}`");
                process::exit(1);
            };
            print!("{text}");
            return;
        }
    }

    let mut project = match Project::try_from_start_dir_and_scan_folders(
        args.common.path.as_deref(),
        args.common.scan_folder.as_deref(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("lele_bevy_lint: {}", lele_lint::Error::with_causes(&e));
            process::exit(1);
        }
    };

    if args.common.sync.sync_config {
        let config_path = ConfigSync::config_path(args.common.config.as_deref(), &project);
        let outcome =
            match ConfigSync::synchronize(&linter.rules, &config_path, args.common.sync.check) {
                Ok(outcome) => outcome,
                Err(e) => {
                    eprintln!("lele_bevy_lint: {}", lele_lint::Error::with_causes(&e));
                    process::exit(1);
                }
            };
        if args.common.sync.check {
            let report = ConfigSync::render_sync_check(&outcome, "lele-bevy-lint");
            if report.is_empty() {
                return;
            }
            eprint!("{report}");
            process::exit(1);
        }
        return;
    }

    let config = match project.load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "lele_bevy_lint: lele.toml: {}",
                lele_lint::Error::with_causes(&e)
            );
            process::exit(1);
        }
    };

    if let Err(e) = project.apply_layout(&config) {
        eprintln!("lele_bevy_lint: {}", lele_lint::Error::with_causes(&e));
        process::exit(1);
    }

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

    let relaxed = match linter.apply_rule_settings(&config) {
        Ok(relaxed) => relaxed,
        Err(e) => {
            eprintln!("lele_bevy_lint: {}", lele_lint::Error::with_causes(&e));
            process::exit(1);
        }
    };

    if args.common.config_report {
        let mut snapshot = snapshot;
        if let Some(codes) = &args.common.only {
            let unknown = linter.retain_codes(codes);
            if !unknown.is_empty() {
                eprintln!(
                    "lele_bevy_lint: no rule with code or name `{}`",
                    unknown.join("`, `")
                );
                process::exit(1);
            }
            snapshot.retain(|entry| linter.rules.iter().any(|rule| rule.name() == entry.name));
        }
        print!("{}", ConfigSync::render_config_report(&snapshot, &relaxed));
        return;
    }

    if let Some(codes) = &args.common.only {
        let unknown = linter.retain_codes(codes);
        if !unknown.is_empty() {
            eprintln!(
                "lele_bevy_lint: no rule with code or name `{}`",
                unknown.join("`, `")
            );
            process::exit(1);
        }
    }
    let relaxed: Vec<lele_lint::RelaxedRule> = relaxed
        .into_iter()
        .filter(|entry| entry.applies_to(&project))
        .collect();
    eprint!("{}", Linter::render_relaxed_banner(&relaxed));

    if args.ui_inventory {
        print!("{}", scan::BevyProject(&project).ui_inventory());
        return;
    }

    process::exit(linter.run(&project));
}
