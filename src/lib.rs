use clap::{Parser, Subcommand};

// strip out usage
const PARSER_TEMPLATE: &str = "{all-args}";
// strip out name/version
const COMMAND_TEMPLATE: &str = "\
     {about-with-newline}\n\
     {usage-heading}\n    {usage}\n\
     \n\
     {all-args}{after-help}\
    ";

#[derive(Debug, Parser)]
#[clap(
    name = "bfbb",
    multicall = true,
    arg_required_else_help = true,
    subcommand_required = true,
    help_template = PARSER_TEMPLATE,
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(subcommand, help_template = COMMAND_TEMPLATE)]
    Spatula(SpatulaCommands),
    #[clap(arg_required_else_help = true, help_template = COMMAND_TEMPLATE)]
    Entity { arg: String },
}

#[derive(Debug, Subcommand)]
pub enum SpatulaCommands {
    List {
        room: Option<String>,
    },
    #[clap(arg_required_else_help = true, help_template=COMMAND_TEMPLATE)]
    Collect {
        spatula: String,
    },
}
