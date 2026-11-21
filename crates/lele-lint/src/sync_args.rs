use clap::Args;

#[derive(Args)]
pub struct SyncArgs {
    #[arg(long)]
    pub sync_config: bool,

    #[arg(long)]
    pub check: bool,
}
