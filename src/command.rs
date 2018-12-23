#[allow(dead_code)]
pub enum InputCommand {
    Uci,
    Debug,
    IsReady,
    SetOption,
    Register,
    UciNewGame,
    Position(String),
    Go,
    Stop,
    PonderHit,
    Quit
}

pub struct CommandResult {
    pub stay: bool,
    pub message: String
}

pub fn send_command(command: InputCommand) -> CommandResult {
    let mut message = String::from("");
    match command {
        InputCommand::Uci => {
            message.push_str(&format!("id name {} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
            message.push_str(&format!("id author {}\n", env!("CARGO_PKG_AUTHORS")));
            message.push_str("uciok\n");
            //todo send optional options
            CommandResult{ stay: true, message: message}
        },
        InputCommand::IsReady => {
            message.push_str("readyok\n"); //wait for initializing => check game state (NOT Searching)
            CommandResult{ stay: true, message: message}
        },
        InputCommand::Quit => CommandResult{ stay: false, message: message},
        _ => CommandResult{ stay: true, message: message} //pretend we know this command :p
    }
}