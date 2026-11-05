pub mod models;
pub mod error;
pub mod file;
pub mod commands;
pub use error::*;
pub use models::*;
pub use utils::command_prelude;
pub use utils::context::GlobalContext;
pub use utils::parsers;
pub mod output;
pub mod utils;

pub use error::{CliError, ValidationErrorKind};
pub use models::{Category, Currency, Record, TrackerData};

use crate::{CliResult, command_prelude::*};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
    vec![
        init::cli(),
        add::cli(),
        list::cli(),
        update::cli(),
        delete::cli(),
        subcategory::cli(),
        total::cli(),
    ]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
    match cmd {
        "init" => Some(init::exec),
        "add" => Some(add::exec),
        "list" => Some(list::exec),
        "update" => Some(update::exec),
        "delete" => Some(delete::exec),
        "subcategory" => Some(subcategory::exec),
        "total" => Some(total::exec),
        _ => None,
    }
}

pub mod init;
pub mod add;
pub mod list;
pub mod update;
pub mod delete;
pub mod subcategory;
pub mod total;
