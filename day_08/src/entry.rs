use std::{cmp::Ordering, fmt, str::FromStr};

pub struct Entry {
    pub unique_signal_patterns: Vec<String>,
    pub output_value: Vec<String>,
}

impl Entry {
    pub fn is_unique_number(num: &String) -> bool {
        if num.len() == 2 {
            // 1
            return true;
        }

        if num.len() == 4 {
            // 4
            return true;
        }

        if num.len() == 3 {
            // 7
            return true;
        }

        if num.len() == 7 {
            // 8
            return true;
        }

        false
    }
    pub fn count_unique_out_numbers(&self) -> usize {
        let mut count = 0;

        for num in self.output_value.iter() {
            if Entry::is_unique_number(num) {
                count += 1;
            }
        }

        count
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split("|").collect();

        // unique signal patterns
        let mut unique_signal_patterns: Vec<String> = split[0]
            .split(" ")
            .filter_map(|s| {
                if s.len() < 1 {
                    // for some reason there is a blank string
                    return None;
                }

                Some(s.to_string())
            })
            .collect();
        unique_signal_patterns.sort_by(|a, b| {
            if a.len() < b.len() {
                Ordering::Less
            } else if a.len() == b.len() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

        // output value
        let output_value: Vec<String> = split[1].split(" ").map(|s| s.to_string()).collect();

        Ok(Entry {
            unique_signal_patterns,
            output_value,
        })
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();

        builder.push_str(
            format!(
                "{} |{}",
                self.unique_signal_patterns.join(" "),
                self.output_value.join(" ")
            )
            .as_str(),
        );

        write!(f, "{}", builder)
    }
}
