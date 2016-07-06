pub enum Command {
    Exit,
    Print(String),
    PrintAttribute(String),
    PrintAdvantage(String),
    SetAttribute(String, i32),
    Damage(u16),
    Help
}



pub fn proccessCommand(command: &str) -> Option<Command> {
    let mut args: Vec<&str> = Vec::new();
    for arg in command.split_whitespace() {
        args.push(arg);
    }
    match args[0] {
        "exit" => Some(Command::Exit),
        "print" => args.get(1).map(|arg| Command::Print(arg.to_string()) ),
        "attribute" => args.get(1).map(|arg| Command::PrintAttribute(arg.to_string())),
        "advantage" => args.get(1).map(|arg| Command::PrintAdvantage(arg.to_string())),
        "set_attribute" => {
            if let Some(name) = args.get(1) {
                if let Some(str_value) = args.get(2) {
                    if let Ok(value) = str_value.parse::<i32>() {
                        Some(Command::SetAttribute(name.to_string(), value))
                    } else { None }
                } else { None }
            } else { None }
        },
        "damage" => args.get(1).map(|arg| Command::Damage(arg.parse::<u16>().unwrap())),
        _ => Some(Command::Help)
    }
}


