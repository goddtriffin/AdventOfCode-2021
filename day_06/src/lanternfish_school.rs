use std::{fmt, str::FromStr};

pub struct LanternfishSchool {
    pub school: [usize; 9],
}

impl LanternfishSchool {
    pub fn simulate(&mut self, num_days: usize) {
        for _ in 0..num_days {
            let mut value_to_use = self.school[8];
            for i in 0..9 {
                let index = 8 - i;

                if index == 0 {
                    self.school[6] += value_to_use;
                    self.school[8] = value_to_use;
                    continue;
                }

                let value_overwritten = self.school[index - 1];
                self.school[index - 1] = value_to_use;
                value_to_use = value_overwritten;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.school.iter().sum()
    }
}

impl FromStr for LanternfishSchool {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let starting_fish: Vec<usize> = s.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

        let mut school = LanternfishSchool { school: [0; 9] };

        for fish in starting_fish.iter() {
            school.school[*fish] += 1;
        }

        Ok(school)
    }
}

impl fmt::Display for LanternfishSchool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();

        for i in 0..9 {
            builder.push_str(format!("{}: {}\n", i, self.school[i]).as_str());
        }

        write!(f, "{}", builder)
    }
}
