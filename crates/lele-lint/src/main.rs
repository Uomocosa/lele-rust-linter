use std::path::Path;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use lele_lint::CommonArgs;
use lele_lint::Config;
use lele_lint::Linter;
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

    let linter = Linter {
        error_format: args.common.error_format,
        ..Linter::default()
    };

    if args.common.checker_list {
        linter.print_checker_list();
        return;
    }

    if args.rules_md {
        print!("{}", linter.rules_markdown());
        return;
    }

    if let Some(code) = args.common.explain {
        let Some(text) = linter.explain(&code) else {
            eprintln!("lele_lint: no rule with code or name `{code}`");
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
            eprintln!("lele_lint: {e}", e = e);
            process::exit(1);
        }
    };

    let config = match resolve_config(args.common.config.as_deref(), &project) {
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
        if let Err(e) = project.sync_methods() {
            eprintln!("lele_lint: {e}", e = e);
            process::exit(1);
        }
        return;
    }

    process::exit(linter.run(&project));
}

// needed helper: a workspace keeps one lele.toml; prefer --config, else CWD, else the crate's own
fn resolve_config(
    explicit: Option<&Path>,
    project: &Project,
) -> Result<(Config, PathBuf), lele_lint::Error> {
    let project_root = project.root.as_path();
    if let Some(path) = explicit {
        return load_at(path, project_root);
    }
    let cwd = std::env::current_dir()
        .ok()
        .map(|dir| dir.join("lele.toml"))
        .filter(|path| path.exists());
    match cwd {
        Some(path) => load_at(&path, project_root),
        None => Ok((project.load_config()?, project_root.to_path_buf())),
    }
}

// needed helper: load one config file and use its directory as the workspace base
fn load_at(path: &Path, fallback: &Path) -> Result<(Config, PathBuf), lele_lint::Error> {
    let path = path.canonicalize().map_err(lele_lint::Error::Io)?;
    let dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| fallback.to_path_buf());
    Ok((Config::try_from(path.as_path())?, dir))
}
