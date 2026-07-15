use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum Commands {
    Ps,
    Inspect { pid: String },
    Tree { root: Option<String> },
    Dot { root: Option<String> },
}
