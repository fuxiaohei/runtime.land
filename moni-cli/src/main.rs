use clap::Parser;

mod flags;

/// cli command line
#[derive(Parser)]
#[clap(name = "moni-cli", version = moni_lib::version::get())]
enum Cli {
    /// Build compiles the project
    Build(flags::Build),
}

#[tokio::main]
async fn main() {
    moni_lib::tracing::init();

    let args = Cli::parse();
    match args {
        Cli::Build(cmd) => cmd.run().await,
    }
}
