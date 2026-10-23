use std::process;

use clap::Parser;
use clap::ValueEnum;
use lele_bevy_lint::BevyLinter;
use lele_bevy_lint::scan;
use lele_lint::CommonArgs;
use lele_lint::Project;

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

    let linter = BevyLinter::from(args.common.error_format);

    if args.common.checker_list {
        linter.print_checker_list();
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

    if let Some(code) = args.common.explain {
        let Some(text) = linter.explain(&code) else {
            eprintln!("lele_bevy_lint: no rule with code or name `{code}`");
            process::exit(1);
        };
        print!("{text}");
        return;
    }

    let mut project = match Project::try_from_start_dir_and_scan_folders(
        args.common.path.as_deref(),
        args.common.scan_folder.as_deref(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("lele_bevy_lint: {}", with_causes(&e));
            process::exit(1);
        }
    };

    let config = match project.load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("lele_bevy_lint: lele.toml: {}", with_causes(&e));
            process::exit(1);
        }
    };

    if let Err(e) = project.apply_layout(&config) {
        eprintln!("lele_bevy_lint: {}", with_causes(&e));
        process::exit(1);
    }

    if args.ui_inventory {
        print!("{}", scan::BevyProject(&project).ui_inventory());
        return;
    }

    process::exit(linter.run(&project));
}

// needed helper: the error and every `source()` cause, joined with ": "
fn with_causes(error: &dyn std::error::Error) -> String {
    let mut message = error.to_string();
    let mut cause = error.source();
    while let Some(inner) = cause {
        message.push_str(": ");
        message.push_str(&inner.to_string());
        cause = inner.source();
    }
    message
}
