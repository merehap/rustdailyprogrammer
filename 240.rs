#![feature(advanced_slice_patterns)]

fn main() {
    let word = scramble_word("Hellow".to_owned());
    println!("{}", word);
}

fn scramble_sentences() {

}

fn fragment_sentences(mut sentences: String) {
    let mut fragments: Vec<SentenceFragment> = vec![];
    while sentences.len() > 0 {
        let text: String = sentences.chars().take_while(|c| c.is_alphabetic()).collect();
        sentences = sentences.chars().skip_while(|c| c.is_alphabetic()).collect();
        let punctuation: String = sentences.chars().take_while(|c| !c.is_alphabetic()).collect();
        sentences = sentences.chars().skip_while(|c| !c.is_alphabetic()).collect();
        fragments.push(SentenceFragment { text: text, punctuation: punctuation });
    }
}

struct SentenceFragment {
    text: String,
    punctuation: String,
}

fn scramble_word(word: String) -> String {
    let chars = word.chars().collect::<Vec<_>>();

    match chars.as_slice() {
        [first, middle .., last] => {
            let mut mid = middle.to_vec();
            for source_index in 0..mid.len() {
                let dest_index = (mid[source_index] as usize) % (mid.len() - source_index) + source_index;
                mid.swap(source_index, dest_index);
            }

            let mut result = vec![first];
            result.append(&mut mid);
            result.push(last);
            result.into_iter().collect()
        },
        _ => word,
    }
}
