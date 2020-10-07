mod program;

mod statics {
    pub mod statements;
}

mod workers {
    pub mod random;
    pub mod guessmachine;
}

use std::io::stdin;
use program::states;
use statics::statements;
use workers::{guessmachine, random};


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
                        println!("nope");
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