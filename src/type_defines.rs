pub enum OperationMode {
    Install, Remove, Search, List, None
}

pub struct CommandArgs {
    pub mode: OperationMode,
    pub args: Vec<String>
}