use std::process;

use clap::Parser;
use clap::ValueEnum;
use lele_bevy_lint::checkers::build_checkers;
use lele_bevy_lint::inventory;
use lele_bevy_lint::skill_markdown;
use lele_lint::CommonArgs;
use lele_lint::Config;
use lele_lint::Project;
use lele_lint::explain;
use lele_lint::print_checker_list;
use lele_lint::rules_markdown;
use lele_lint::run_checkers;

#[derive(Clone, Copy, ValueEnum)]
enum Emit {
    RulesMd,
    SkillMd,
}

#[derive(Parser)]
#[command(
    name = "lele_bevy_lint",
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

    if args.common.checker_list {
        print_checker_list(&build_checkers());
        return;
    }

    match args.emit {
        Some(Emit::RulesMd) => {
            print!("{}", rules_markdown(&build_checkers()));
            return;
        }
        Some(Emit::SkillMd) => {
            print!("{}", skill_markdown());
            return;
        }
        None => {}
    }

    if let Some(code) = args.common.explain {
        let Some(text) = explain(&build_checkers(), &code) else {
            eprintln!("lele_bevy_lint: no rule with code or name `{code}`");
            process::exit(1);
        };
        print!("{text}");
        return;
    }

    let mut project = match Project::discover(
        args.common.path.as_deref(),
        args.common.scan_folder.as_deref(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("lele_bevy_lint: {e}", e = e);
            process::exit(1);
        }
    };

    let config = match Config::load(&project.root) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("lele_bevy_lint: lele.toml: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = project.apply_layout(&config) {
        eprintln!("lele_bevy_lint: {e}", e = e);
        process::exit(1);
    }

    if args.ui_inventory {
        print!("{}", inventory::report(&project));
        return;
    }

    let checkers = build_checkers();

    process::exit(run_checkers(&project, &checkers, args.common.error_format));
}
