use std::fmt;

use crate::entry::Entry;

pub struct SevenSegmentDisplay {
    a: Vec<char>, // top
    b: Vec<char>, // top right
    c: Vec<char>, // bottom right
    d: Vec<char>, // bottom
    e: Vec<char>, // bottom left
    f: Vec<char>, // top left
    g: Vec<char>, // middle
}

impl SevenSegmentDisplay {
    pub fn new(entry: &Entry) -> Self {
        let mut display = SevenSegmentDisplay {
            a: vec![],
            b: vec![],
            c: vec![],
            d: vec![],
            e: vec![],
            f: vec![],
            g: vec![],
        };

        for digit in entry.unique_signal_patterns.iter() {
            display.store_possible_wire_positions(digit);
            println!();
        }

        display
    }

    fn store_possible_wire_positions(&mut self, digit: &String) {
        println!("digit: {}", digit);

        // digits: 1
        if digit.len() == 2 {
            println!("found: 1");
            self.store_digit_wires(digit, &vec!['b', 'c']);
            return;
        }

        // digits: 4
        if digit.len() == 4 {
            println!("found: 4");
            self.store_digit_wires(digit, &vec!['b', 'c', 'f', 'g']);
            return;
        }

        // digits: 7
        if digit.len() == 3 {
            println!("found: 7");
            self.store_digit_wires(digit, &vec!['a', 'b', 'c']);
            return;
        }

        // digits: 8
        if digit.len() == 7 {
            println!("found: 8");
            self.store_digit_wires(digit, &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
            return;
        }

        // digits: 2, 3, 5
        if digit.len() == 5 {
            println!("found: 2/3/5");
            self.store_digit_wires(digit, &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
            return;
        }

        // digits: 0, 6, 9
        if digit.len() == 6 {
            println!("found: 0/6/9");
            self.store_digit_wires(digit, &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
            return;
        }
    }

    fn store_digit_wires(&mut self, digit: &String, locations: &Vec<char>) {
        let mut unused_wires: Vec<char> = vec![];
        let mut locations = locations.clone();

        for wire in digit.chars() {
            let already_stored_locations = self.get_wire_already_stored_locations(&wire);
            if already_stored_locations.len() == 0 {
                unused_wires.push(wire);
                continue;
            }

            locations = locations
                .into_iter()
                .filter(|location| !already_stored_locations.contains(location))
                .collect();
        }

        println!(
            "unused wires: {}",
            unused_wires
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "locations: {}",
            locations
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        for wire in unused_wires.iter() {
            self.store_wire(&locations, wire);
        }
    }

    fn get_wire_already_stored_locations(&self, wire: &char) -> Vec<char> {
        let mut already_stored_locations: Vec<char> = vec![];

        if self.a.contains(wire) {
            already_stored_locations.push('a');
        }
        if self.b.contains(wire) {
            already_stored_locations.push('b');
        }
        if self.c.contains(wire) {
            already_stored_locations.push('c');
        }
        if self.d.contains(wire) {
            already_stored_locations.push('d');
        }
        if self.e.contains(wire) {
            already_stored_locations.push('e');
        }
        if self.f.contains(wire) {
            already_stored_locations.push('f');
        }
        if self.g.contains(wire) {
            already_stored_locations.push('g');
        }

        already_stored_locations
    }

    fn store_wire(&mut self, locations: &Vec<char>, wire: &char) {
        for location in locations.iter() {
            if *location == 'a' {
                println!("storing {} in {}", wire, 'a');
                self.a.push(*wire);
                continue;
            }
            if *location == 'b' {
                println!("storing {} in {}", wire, 'b');
                self.b.push(*wire);
                continue;
            }
            if *location == 'c' {
                println!("storing {} in {}", wire, 'c');
                self.c.push(*wire);
                continue;
            }
            if *location == 'd' {
                println!("storing {} in {}", wire, 'd');
                self.d.push(*wire);
                continue;
            }
            if *location == 'e' {
                println!("storing {} in {}", wire, 'e');
                self.e.push(*wire);
                continue;
            }
            if *location == 'f' {
                println!("storing {} in {}", wire, 'f');
                self.f.push(*wire);
                continue;
            }
            if *location == 'g' {
                println!("storing {} in {}", wire, 'g');
                self.g.push(*wire);
                continue;
            }
        }
    }
}

impl fmt::Display for SevenSegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = String::new();

        builder.push_str(
            format!(
                "a: [{}]\n",
                self.a
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );
        builder.push_str(
            format!(
                "b: [{}]\n",
                self.b
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );
        builder.push_str(
            format!(
                "c: [{}]\n",
                self.c
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );
        builder.push_str(
            format!(
                "d: [{}]\n",
                self.d
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );
        builder.push_str(
            format!(
                "e: [{}]\n",
                self.e
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );
        builder.push_str(
            format!(
                "f: [{}]\n",
                self.f
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );
        builder.push_str(
            format!(
                "g: [{}]\n",
                self.g
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .as_str(),
        );

        write!(f, "{}", builder)
    }
}
