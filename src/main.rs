use bfbb_cli::{Cli, Commands, SpatulaCommands};
use clap::Parser;

use std::{
    io::{self, Write},
    process,
};

fn main() {
    ctrlc::set_handler(|| process::exit(0)).expect("Error setting ctrl-c handler");

    println!(
        "Welcome to the BfBB interactive shell\n\
    Press Ctrl-C to exit.\n"
    );

    let mut buf = String::new();
    loop {
        print!("> {buf}");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line from stdin");
        let args = Cli::try_parse_from(shlex::split(buf.trim_end()).unwrap());

        if let Err(e) = args {
            println!("{e}");
            continue;
        }
        buf.clear();
        let args = args.unwrap();

        match args.command {
            Commands::Spatula(sub) => match sub {
                SpatulaCommands::List { room } => match room {
                    Some(val) => println!("Listing {val}"),
                    None => println!("Listing All"),
                },
                SpatulaCommands::Collect { spatula } => {
                    println!("Collecting {spatula}")
                }
            },
            Commands::Entity { arg } => {
                println!("Doing {arg} to entity");
            }
        }
    }
}
