use std::io::Write;

pub fn input(_headline: &str) -> &str{
    print!("{}", _headline);
    std::io::stdout().flush().unwrap(); // idk what this thing does dont remove it
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).expect("err"); // take input
    _input.pop(); // pop the last index ("\n")
    let _str_input: &str = Box::leak(_input.into_boxed_str()); // convert String to &str
    return _str_input;
}
