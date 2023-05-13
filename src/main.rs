use std::env;

enum Mode {
    Install, Remove, Search, List, None
}

struct CommandArgs {
    mode: Mode,
    args: Vec<String>
}

fn main() {
    // 引数解析
    let mode = env::args().nth(1).unwrap_or(String::from("__NONE__"));
    let cmd_args = CommandArgs {
        mode: match &*mode {
            "install" | "i" => Mode::Install,
            "remove" | "r" => Mode::Remove,
            "search" | "s" => Mode::Search,
            "list" | "l" => Mode::List,
            _ => Mode::None
        },
        args: env::args().skip(2).collect()
    };

    
    return
}
