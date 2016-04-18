use std::collections::HashMap;

fn main() {
    let texts = vec![
        "122333444455555666666777777788888888",
        "563881467447538846567288767728553786",
        "https://www.reddit.com/r/dailyprogrammer",
        "int main(int argc, char *argv[])"
    ];

    for text in texts {
        println!("Entropy {} for {}", entropy(text.to_owned()), text);
    }
}

fn entropy(text: String) -> f32 {
    let mut frequencies = HashMap::new();
    for c in text.chars() {
        let point = c as u8;
        if frequencies.contains_key(&point) {
            *frequencies.get_mut(&point).unwrap() += 1.0;
        } else {
            frequencies.insert(point, 1.0);
        }
    };

    -1.0 * frequencies.iter().map(|(_, &count)| {
        let text_len = text.len() as f32;
        (count / text_len) * (count / text_len).log(2.0)
    }).fold(0.0, |sum, c| sum + c)
}
