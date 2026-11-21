use std::process;

use clap::Parser;
use clap::ValueEnum;
use lele_bevy_lint::BevyLinter;
use lele_bevy_lint::scan;
use lele_lint::CommonArgs;
use lele_lint::LintSession;

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

    let ui_inventory = args.ui_inventory;
    process::exit(
        LintSession {
            binary: "lele_bevy_lint",
        }
        .run(
            &mut linter,
            &args.common,
            false,
            |_, _| Ok(()),
            move |project| {
                ui_inventory.then(|| {
                    print!("{}", scan::BevyProject(project).ui_inventory());
                    0
                })
            },
        ),
    );
}
