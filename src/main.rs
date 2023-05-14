// use types::type_defines::{CommandArgs, OperationMode};
use balmuda::type_defines::{CommandArgs, OperationMode};
use std::env;

mod install;

fn main() {
    // 引数解析
    let mode = env::args().nth(1).unwrap_or(String::from("__NONE__"));
    let cmd_args = CommandArgs {
        mode: match &*mode {
            "install" | "i" => OperationMode::Install,
            "remove" | "r" => OperationMode::Remove,
            "search" | "s" => OperationMode::Search,
            "list" | "l" => OperationMode::List,
            _ => OperationMode::None
        },
        args: env::args().skip(2).collect()
    };

    
    return
}
