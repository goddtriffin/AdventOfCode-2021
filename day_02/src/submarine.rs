use crate::command::Command;

pub struct Submarine {
    // part 1
    pub horizontal_position: i32,
    pub depth: i32,

    // part 2
    pub aim: i32,
    pub aim_horizontal_position: i32,
    pub aim_depth: i32,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine {
            // part 1
            horizontal_position: 0,
            depth: 0,

            // part 2
            aim: 0,
            aim_horizontal_position: 0,
            aim_depth: 0,
        }
    }

    pub fn travel(&mut self, commands: &Vec<Command>) {
        for command in commands {
            match command {
                Command::Forward(f) => {
                    // part 1
                    self.horizontal_position += f;

                    // part 2
                    self.aim_horizontal_position += f;
                    self.aim_depth += f * self.aim;
                }
                Command::Up(u) => {
                    // part 1
                    self.depth -= u;

                    // part 2
                    self.aim -= u;
                }
                Command::Down(d) => {
                    // part 1
                    self.depth += d;

                    // part 2
                    self.aim += d;
                }
            }
        }
    }

    pub fn multiply(&self) -> i32 {
        self.horizontal_position * self.depth
    }

    pub fn aim_multiply(&self) -> i32 {
        self.aim_horizontal_position * self.aim_depth
    }
}
