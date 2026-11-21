use std::process;

use clap::Parser;
use lele_lint::CommonArgs;
use lele_lint::LintSession;
use lele_lint::Linter;

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

    process::exit(
        LintSession {
            binary: "lele_lint",
        }
        .run(
            &mut linter,
            &args.common,
            args.sync_methods,
            |_, _| Ok(()),
            |_| None,
        ),
    );
}
