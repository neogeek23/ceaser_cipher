use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let prompt = String::from("test:  ");
    let s:String = get_string_from_console_prompt(prompt);
    println!("You entered: {}", s);
}

fn get_string_from_console_prompt(prompt:String) -> String{
    let mut s=String::new();
    println!("{}", prompt);
    stdin().read_line(&mut s).expect("Input Error: Incorrect string.");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s.to_string()
}