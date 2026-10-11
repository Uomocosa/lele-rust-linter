use std::path::PathBuf;

use clap::Args;

use crate::ErrorFormat;

#[derive(Args)]
pub struct CommonArgs {
    #[arg(short, long, value_enum, default_value_t = ErrorFormat::Clippy)]
    pub error_format: ErrorFormat,

    #[arg(long)]
    pub checker_list: bool,

    #[arg(long, value_name = "CODE")]
    pub explain: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    #[arg(long = "scan-folder", value_name = "FOLDERS", value_delimiter = ',')]
    pub scan_folder: Option<Vec<PathBuf>>,

    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::CommonArgs;
    use clap::Parser;

    #[derive(Parser)]
    struct Harness {
        #[command(flatten)]
        common: CommonArgs,
    }

    #[test]
    fn test_usage() {
        let parsed = Harness::try_parse_from(["x", "--scan-folder", "src,tests", "here"]).unwrap();
        assert_eq!(parsed.common.scan_folder.map(|f| f.len()), Some(2));
        assert_eq!(parsed.common.path, Some("here".into()));
    }
}
