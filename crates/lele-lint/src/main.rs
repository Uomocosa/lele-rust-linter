use std::path::Path;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use lele_lint::checkers::build_checkers;
use lele_lint::explain;
use lele_lint::print_checker_list;
use lele_lint::rules_markdown;
use lele_lint::run_checkers;
use lele_lint::sync_methods;
use lele_lint::CommonArgs;
use lele_lint::Config;
use lele_lint::Project;

#[derive(Parser)]
#[command(name = "lele_lint", about = "Enforce lele-syntax-rs conventions")]
struct Args {
    #[command(flatten)]
    common: CommonArgs,

    #[arg(long = "rules-md")]
    rules_md: bool,

    #[arg(long = "sync-methods")]
    sync_methods: bool,
}

fn main() {
    let args = Args::parse();

    if args.common.checker_list {
        let checkers = build_checkers();
        print_checker_list(&checkers);
        return;
    }

    if args.rules_md {
        print!("{}", rules_markdown(&build_checkers()));
        return;
    }

    if let Some(code) = args.common.explain {
        let Some(text) = explain(&build_checkers(), &code) else {
            eprintln!("lele_lint: no rule with code or name `{code}`");
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
            eprintln!("lele_lint: {e}", e = e);
            process::exit(1);
        }
    };

    let config = match resolve_config(args.common.config.as_deref(), &project.root) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("lele_lint: lele.toml: {e}");
            process::exit(1);
        }
    };
    let (config, config_dir) = config;
    project.config_dir = config_dir;

    if let Err(e) = project.apply_layout(&config) {
        eprintln!("lele_lint: {e}", e = e);
        process::exit(1);
    }

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
        if let Err(e) = sync_methods(&project) {
            eprintln!("lele_lint: {e}", e = e);
            process::exit(1);
        }
        return;
    }

    let checkers = build_checkers();

    process::exit(run_checkers(&project, &checkers, args.common.error_format));
}

// needed helper: a workspace keeps one lele.toml; prefer --config, else CWD, else the crate's own
fn resolve_config(
    explicit: Option<&Path>,
    project_root: &Path,
) -> Result<(Config, PathBuf), lele_lint::Error> {
    if let Some(path) = explicit {
        return load_at(path, project_root);
    }
    let cwd = std::env::current_dir()
        .ok()
        .map(|dir| dir.join("lele.toml"))
        .filter(|path| path.exists());
    match cwd {
        Some(path) => load_at(&path, project_root),
        None => Ok((Config::load(project_root)?, project_root.to_path_buf())),
    }
}

// needed helper: load one config file and use its directory as the workspace base
fn load_at(path: &Path, fallback: &Path) -> Result<(Config, PathBuf), lele_lint::Error> {
    let path = path.canonicalize().map_err(lele_lint::Error::Io)?;
    let dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| fallback.to_path_buf());
    Ok((Config::load_file(&path)?, dir))
}
