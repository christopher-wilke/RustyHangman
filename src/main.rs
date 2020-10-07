mod program;

mod statics {
    pub mod statements;
}

mod workers {
    pub mod guessmachine;
    pub mod hangmanwriter;
    pub mod random;
}

use std::io::stdin;
use program::States;
use statics::statements;
use workers::{guessmachine, hangmanwriter, random};


fn main() {

    println!("{}", statements::program_starts());

    let mut input = String::new();
    let mut buffer = String::new();
    let word_to_guess: String = random::get_word();
    let mut guesser = guessmachine::create(word_to_guess);
    guessmachine::print_current_state(&guesser);
    let mut program_state = States::UserPressedKey;

    loop 
    {
        buffer.clear();

        match program_state
        {
            States::UserPressedKey =>
            {
                if &input.len() > &0
                {
                    let success: bool = guessmachine::user_pressed_key(&input, &mut guesser);

                    if success
                    {
                        guessmachine::print_current_state(&guesser);
                    }
                    else
                    {
                        match guessmachine::get_counter_wrong_chars(&guesser)
                        {
                            1 => hangmanwriter::first_attempt(),
                            2 => hangmanwriter::second_attempt(),
                            3 => hangmanwriter::third_attempt(),
                            4 => hangmanwriter::fourth_attempt(),
                            _ => println!("Game ended.")
                        }
                    }
                    &input.clear();
                }
            },
            States::ProgramExits => {
                println!("Program exits");
                break;
            }
        }

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                input.push_str(&buffer.trim_end().to_lowercase());

                match &*input {
                    "exit" => program_state = States::ProgramExits,
                    _ => program_state = States::UserPressedKey
                }
            },
            Err(_) => continue 
        }
    }
}