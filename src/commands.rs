use clap::Subcommand;

mod dot;
mod inspect;
mod ps;
mod tree;

#[derive(Subcommand)]
pub enum Commands {
    Ps,
    Inspect { pid: String },
    Tree { root: Option<String> },
    Dot { root: Option<String> },
}
