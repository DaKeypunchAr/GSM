pub enum Command {
    Help,
    ShowDealers,
    AddDealer,
    ShowGoods,
    AddGood,
    SaveData,
    Exit,
}

impl Command {
    pub fn build(str: &str) -> Option<Command> {
        let str = str.trim();

        if let Ok(x) = str.parse::<u8>() {
            return Self::u8_to_cmd(x);
        }

        Self::str_to_cmd(str)
    }

    pub fn u8_to_cmd(num: u8) -> Option<Command> {
        match num {
            1 => Some(Command::Help),
            2 => Some(Command::ShowDealers),
            3 => Some(Command::AddDealer),
            4 => Some(Command::ShowGoods),
            5 => Some(Command::AddGood),
            6 => Some(Command::SaveData),
            0 => Some(Command::Exit),
            _ => None,
        }
    }

    pub fn str_to_cmd(str: &str) -> Option<Command> {
        match str.to_lowercase().as_str() {
            "help" => Some(Command::Help),
            "show dealers" => Some(Command::ShowDealers),
            "add dealer" => Some(Command::AddDealer),
            "show goods" => Some(Command::ShowGoods),
            "add good" => Some(Command::AddGood),
            "save data" => Some(Command::SaveData),
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Help => "help",
            Self::ShowDealers => "show dealers",
            Self::AddDealer => "add dealer",
            Self::ShowGoods => "show goods",
            Self::AddGood => "add good",
            Self::SaveData => "save data",
            Self::Exit => "exit",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Help => 1,
            Self::ShowDealers => 2,
            Self::AddDealer => 3,
            Self::ShowGoods => 4,
            Self::AddGood => 5,
            Self::SaveData => 6,
            Self::Exit => 0,
        }
    }

    pub fn cmds() -> [Command; 7] {
        const COMMANDS: [Command; 7] = [
            Command::Help,
            Command::ShowDealers,
            Command::AddDealer,
            Command::ShowGoods,
            Command::AddGood,
            Command::SaveData,
            Command::Exit,
        ];
        COMMANDS
    }
}
