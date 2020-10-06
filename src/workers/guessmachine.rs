#[derive(Debug)]
pub struct GuessState
{
    current_guess_state: Vec<String>,
    counter_wrong_chars: usize,
    word_to_guess: &'static str
}

trait GuessMachine
{
    fn create(word_to_guess: &'static str) -> Self;
    fn user_pressed_key(value: &String, state: &GuessState);
}

impl GuessMachine for GuessState
{
    fn create(word_to_guess: &'static str) -> Self
    {
        let mut _guess_state: Vec<String> = Vec::new();
        for _c in word_to_guess.chars()
        {
            _guess_state.push("_".to_string());
        }

        GuessState
        {
            current_guess_state: _guess_state,
            counter_wrong_chars: 0,
            word_to_guess: word_to_guess,
        }
    }

    fn user_pressed_key(value: &String, state: &Self)
    {
        let mut i: usize = 0;

        for _c in state.word_to_guess.chars()
        {
            match state.word_to_guess.chars().nth(i)
            {
                Some(x) => {
                    if let Some(y) = value.chars().nth(0)
                    {
                        if y == x
                        {
                            println!("We have a match at {}! {}", i, y);
                        }
                    }
                },
                None => println!("Could not find value")
            };

            i += 1;
        }
        println!("You pressed {}", *value);
        // println!("{}", "Hello rfrom user pressed event");
    }
}

pub fn user_pressed_key(value: &String, guess_state: &GuessState)
{
    GuessState::user_pressed_key(value, &guess_state);
}

pub fn create(input: String) -> GuessState
{
    let boxed_string: &'static str = Box::leak(input.into_boxed_str());
    GuessState::create(boxed_string)
}

pub fn print_current_state(state: &GuessState)
{
    let mut output = String::new();

    for c in &state.current_guess_state
    {
        output.push_str(c);
    }

    println!("{}", output);
}