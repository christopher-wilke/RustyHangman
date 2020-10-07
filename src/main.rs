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
use program::states;
use statics::statements;
use workers::{guessmachine, hangmanwriter, random};


fn main() {

    let mut input = String::new();
    let mut buffer = String::new();

    println!("{}", statements::program_starts());
    let word_to_guess: String = random::get_word();

    println!("word to guess = {}", &word_to_guess);
    let mut guesser = guessmachine::create(word_to_guess);
    guessmachine::print_current_state(&guesser);
    let mut program_state = states::UserPressedKey;

    loop 
    {
        buffer.clear();

        match program_state
        {
            states::UserPressedKey =>
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
                            1 => workers::hangmanwriter::first_attempt(),
                            2 => workers::hangmanwriter::second_attempt(),
                            3 => workers::hangmanwriter::third_attempt(),
                            4 => workers::hangmanwriter::fourth_attempt(),
                            _ => println!("Game ended.")
                        }
                    }
                    &input.clear();
                }
            },
            states::ProgramExits => {
                println!("Program exits");
                break;
            }
        }

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                input.push_str(&buffer.trim_end().to_lowercase());

                match &*input {
                    "exit" => program_state = states::ProgramExits,
                    _ => program_state = states::UserPressedKey
                }
            },
            Err(_) => continue 
        }
    }
}