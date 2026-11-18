use std::path::PathBuf;

use clap::Args;

use crate::ErrorFormat;

#[derive(Args)]
pub struct CommonArgs {
    #[arg(short, long, value_enum, default_value_t = ErrorFormat::Clippy)]
    pub error_format: ErrorFormat,

    #[arg(long)]
    pub rule_list: bool,

    #[arg(long, value_name = "CODE")]
    pub explain: Option<String>,

    #[arg(long, value_name = "CODES", value_delimiter = ',')]
    pub only: Option<Vec<String>>,

    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    #[arg(long = "scan-folder", value_name = "FOLDERS", value_delimiter = ',')]
    pub scan_folder: Option<Vec<PathBuf>>,

    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,
}
