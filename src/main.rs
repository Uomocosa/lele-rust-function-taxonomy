use std::path::PathBuf;

use clap::Parser;

use lele_rust_function_taxonomy::{run, FailureReason, Outcome};

#[derive(Parser, Debug)]
#[command(
    name = "lele-function-taxonomy",
    about = "Checks that functions in require=honest boundary folders are honest (rustc MIR)"
)]
struct Args {
    #[arg(long)]
    manifest_path: Option<PathBuf>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let code = match run(args.manifest_path) {
        Outcome::NothingToCheck => {
            println!("no honest boundaries configured — nothing to check");
            0
        }
        Outcome::Clean => 0,
        Outcome::Failed(FailureReason::CargoCheckFailed) => 1,
        Outcome::Failed(reason) => {
            eprintln!("{reason}");
            1
        }
    };
    std::process::exit(code);
}
