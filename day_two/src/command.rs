use std::str::FromStr;

pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(' ').collect();
        if split.len() != 2 {
            return Err(());
        }

        let value: i32 = split[1].parse::<i32>().unwrap();

        match split[0] {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err(()),
        }
    }
}
