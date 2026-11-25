//! The `aos` command-line entry point.
//!
//! Stage 0 exposes only the version and help. Subcommands arrive with the
//! adaptive runtime (Stage 1); see `docs/12-roadmap.md`.

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        None | Some("-h" | "--help") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("-V" | "--version") => {
            println!("aos {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("aos: unknown command '{other}'\n");
            print_help();
            ExitCode::FAILURE
        }
    }
}

/// Prints usage. The `aos` binary only reports its version at Stage 0.
fn print_help() {
    println!(
        "aos {version}\n\
         Xdaptive OS — Beyond Applications.\n\
         \n\
         USAGE:\n    aos [OPTIONS]\n\
         \n\
         OPTIONS:\n    -h, --help       Print this help\n    -V, --version    Print the version\n\
         \n\
         Stage 0 exposes only the version and help. Subcommands arrive with the\n\
         adaptive runtime; see docs/12-roadmap.md.",
        version = env!("CARGO_PKG_VERSION"),
    );
}
