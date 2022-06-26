mod handlers;

use bfbb_cli::{Cli, Commands, SpatulaCommands};
use clap::Parser;
use handlers::CtrlCHandler;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    println!("Welcome to the BfBB interactive shell");

    let mut rl = Editor::<()>::new();
    rl.bind_sequence(rustyline::KeyEvent::ctrl('c'), CtrlCHandler);
    loop {
        // let readline = rl.readline(">> ");
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                let args = Cli::try_parse_from(shlex::split(&line).unwrap_or_default());
                if let Err(e) = args {
                    println!("{e}");
                    continue;
                }
                let args = args.unwrap();

                match args.command {
                    Commands::Entity { arg } => {
                        println!("Doing {arg} to entity");
                    }
                    Commands::Hans => println!("Toggling Hans"),
                    Commands::Interface => {
                        println!("Game Interface info here.")
                    }
                    Commands::Player(sub) => match sub {
                        bfbb_cli::PlayerCommands::Switch => println!("Switching character"),
                        bfbb_cli::PlayerCommands::BubbleBowl => println!("Toggling bubble bowl"),
                        bfbb_cli::PlayerCommands::CruiseBubble => {
                            println!("Toggling cruise bubble")
                        }
                    },
                    Commands::Spatula(sub) => match sub {
                        SpatulaCommands::List { level } => match level {
                            Some(val) => println!("Listing {val}"),
                            None => println!("Listing All"),
                        },
                        SpatulaCommands::Collect { spatula } => {
                            println!("Collecting {spatula}")
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
