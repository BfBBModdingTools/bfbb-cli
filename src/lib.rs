use clap::{Parser, Subcommand};

// strip out usage
const PARSER_TEMPLATE: &str = "{all-args}";
// strip out name/version
const COMMAND_TEMPLATE: &str = "\
     {about-with-newline}\n\
     {usage-heading}\t{usage}\n\
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
    #[clap(arg_required_else_help = true, help_template = COMMAND_TEMPLATE)]
    /// Perform actions based on entities
    Entity { arg: String },
    /// Toggle Hans on or off
    Hans,
    /// Actions and information related to the game interface
    Interface,
    #[clap(subcommand, help_template = COMMAND_TEMPLATE)]
    /// Actions on the player
    Player(PlayerCommands),
    #[clap(subcommand, help_template = COMMAND_TEMPLATE)]
    /// Perform actions based on spatulas
    Spatula(SpatulaCommands),
}

#[derive(Debug, Subcommand)]
pub enum PlayerCommands {
    /// Switch to alternative character.
    Switch,
    /// Toggle the bubble bowl
    BubbleBowl,
    /// Toggle the cruise bubble
    CruiseBubble,
}

#[derive(Debug, Subcommand)]
pub enum SpatulaCommands {
    /// List spatulas names from a given Level. If no Level is given, list spatulas names in the current level
    List {
        /// List spatulas from this Level only
        level: Option<String>,
    },
    #[clap(arg_required_else_help = true, help_template=COMMAND_TEMPLATE)]
    /// Collect a spatula
    Collect { spatula: String },
}
