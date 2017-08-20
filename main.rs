use std::io::stdin;

fn main() {
    let mut prompt = String::from("What message would you like to encrypt:  ");
    let message:String = get_string_from_console_prompt(prompt);
    prompt = String::from("What shift would you like to have for your message?");
    let shift:u8 = get_string_from_console_prompt(prompt).parse().unwrap();
    println!("Ceaser Cipher is:  {}", ceaser_cipher(message, shift));
//    print_all_the_characters();
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

fn ceaser_cipher(message:String, shift:u8) -> String{
    let mut s = String::new();
    let char_array = message.as_bytes();

    for character in char_array{
        //let character_u32 = u32::From(character_u8);\
        let result:u16 = (*character as u16) + (shift as u16);
        let c = ((result as u8) % (<u8>::max_value() as u8)) as char;
        s.push(c);
    }
    s
}

fn print_all_the_characters(){
    let mut var:u8 = 0;

    while var < (<u8>::max_value() as u8){
        let c = var as char;
        println!("{})\t{}", var, c);
        var += 1;
    }
}