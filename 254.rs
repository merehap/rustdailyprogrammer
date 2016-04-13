
fn main() {

    let inputs = vec!["foobar", "wizard", "/r/dailyprogrammer", "gsrh rh zm vcznkov lu gsv zgyzhs xrksvi"];
    for input in inputs {
        println!("{}", atbash(input.to_owned()));
    }
}

fn atbash(text: String) -> String {
    let mut cipher_text = "".to_owned();
    for letter in text.chars() {
        let rev = match letter {
            'a'...'z' => (25 - ((letter as u8) - ('a' as u8)) + ('a' as u8)) as char,
            _ => letter,
        };

        cipher_text.push(rev);
    }

    cipher_text
}
