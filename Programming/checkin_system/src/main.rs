use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Deserialize)]
struct Participant {
    name: String,
    age: u8,
    number: u32,
}

struct CheckinSystem {
    participants: Vec<Participant>,
}

impl CheckinSystem {
    fn new(list_file: &str) -> Self {
        let file_content = fs::read_to_string(list_file)
            .expect("Couldn't read file.");
        let participants: Vec<Participant> = serde_json::from_str(&file_content)
            .expect("Failed to parse JSON data.");

        CheckinSystem { participants }
    }

    fn checkin(&self) {
        println!("This is the checkin system.");

        loop {
            println!("\nType your name ('exit' to quit):");

            let mut name_input = String::new();
            io::stdin()
                .read_line(&mut name_input)
                .expect("Couldn't read input.");

            let name_input = name_input.trim();

            if name_input.eq_ignore_ascii_case("exit") {
                println!("Exiting the program.");
                break;
            }

            match self.participants.iter().find(|participant| participant.name.eq_ignore_ascii_case(name_input)) {
                Some(participant) => {
                    println!("Welcome, {}! Your check-in is completed.", participant.name);
                    println!("Your lotto number is {}. Good luck!", participant.number);
                }
                None => {
                    println!("The name '{}' is not in the list. Please try again.", name_input);
                }
            }
        }
    }
}

fn main() {
    let system = CheckinSystem::new("data/list.json");
    system.checkin();
}

