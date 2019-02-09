use crate::command::InputCommand;

pub fn parse(line: &str) -> Option<InputCommand> {
    let words = line.split_whitespace().collect::<Vec<&str>>();

    for (i, &word) in words.iter().enumerate() {
        match word {
            "uci" => return Some(InputCommand::Uci),
            "debug" => return Some(InputCommand::Debug),
            "isready" => return Some(InputCommand::IsReady),
            "setoption" => return Some(InputCommand::SetOption),
            "register" => return Some(InputCommand::Register),
            "ucinewgame" => return Some(InputCommand::UciNewGame),
            "position" => return Some(InputCommand::Position(get_rest_string(&words, i))),
            "go" => return Some(InputCommand::Go(get_rest_string(&words, i))),
            "stop" => return Some(InputCommand::Stop),
            "ponderhit" => return Some(InputCommand::PonderHit),
            "quit" => return Some(InputCommand::Quit),
            _ => continue
        }
    }

    return None;
}

fn get_rest_string(words: &Vec<&str>, i: usize) -> String {
    if words.len() > (i + 1) {
        return words[i + 1 ..].join(" ")
    }
    "".to_string()
}