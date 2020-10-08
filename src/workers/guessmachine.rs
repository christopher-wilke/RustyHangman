pub struct GuessState
{
    current_guess_state: Vec<String>,
    counter_wrong_chars: usize,
    word_to_guess: &'static str
}

trait GuessMachine
{
    fn create(word_to_guess: &'static str) -> Self;
    fn user_pressed_key(&mut self, value: &String) -> bool;
    fn add_to_counter_wrong_chars(&mut self);
    fn get_counter_wrong_chars(&self) -> usize;
    fn finished(&self);
}

impl GuessMachine for GuessState
{
    fn finished(&self)
    {
        let mut finished: bool = true;

        for x in self.current_guess_state.iter()
        {
            if x == "_"
            {
                finished = false;
            }
        }
        if finished == true
        {
            println!("Congratulations! You guessed it right :)");
        }
    }

    fn get_counter_wrong_chars(&self) -> usize
    {
        self.counter_wrong_chars
    }

    fn add_to_counter_wrong_chars(&mut self)
    {
        self.counter_wrong_chars += 1;
    }

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
            word_to_guess: word_to_guess
        }
    }

    fn user_pressed_key(&mut self, value: &String) -> bool
    {
        let mut i: usize = 0;
        let mut hit: bool = false;

        for _c in self.word_to_guess.chars()
        {

            if let Some(x) = self.word_to_guess.chars().nth(i)
            {
                if let Some(y) = value.chars().nth(0)
                {
                    if y == x
                    {                           
                        self.current_guess_state[i] = y.to_string();
                        hit = true;
                    }
                }
            }
            i += 1;
        }
        if hit == false
        {
            self.add_to_counter_wrong_chars();
        }
        self.finished();

        hit
    }
}

pub fn get_counter_wrong_chars(state: &GuessState) -> usize
{
    state.get_counter_wrong_chars()
}

pub fn user_pressed_key(state: &mut GuessState, value: &String) -> bool
{
    state.user_pressed_key(value)
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