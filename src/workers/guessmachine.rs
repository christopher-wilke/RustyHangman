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
    fn user_pressed_key(value: &String, state: &mut GuessState) -> bool;
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

    fn user_pressed_key(value: &String, state: &mut GuessState) -> bool
    {
        let mut i: usize = 0;
        let mut hit: bool = false;

        for _c in state.word_to_guess.chars()
        {
            match state.word_to_guess.chars().nth(i)
            {
                Some(x) => {
                    if let Some(y) = value.chars().nth(0)
                    {
                        if y == x
                        {                           
                            state.current_guess_state[i] = y.to_string();
                            hit = true;
                        }
                    }
                },
                None => println!("Could not find value")
            };
            i += 1;
        }

        hit
    }
}

pub fn user_pressed_key(value: &String, state: &mut GuessState) -> bool
{
    GuessState::user_pressed_key(value, state)
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