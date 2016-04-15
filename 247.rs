use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("247.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);

    let empty_list = vec![];
    let mut name_pairs = input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .zip(0..)
        .fold(empty_list, |mut names, (family, index)| {
            names.append(&mut family.iter()
                .map(|&name| (index, name))
                .collect::<Vec<_>>());
            names
        });

    // Rust doesn't include a random number generator in its standard library yet
    // and this project isn't set up with Cargo so I can't import one.
    let mut random_numbers = random_numbers();

    let givers = name_pairs.clone();
    for (family_index, giver) in givers {
        let temp_name_pairs = name_pairs.clone();
        let candidate_pairs = temp_name_pairs.iter()
            .filter(|&&(index, _)| index != family_index)
            .collect::<Vec<_>>();
        let target_index = random_numbers.remove(0) % candidate_pairs.len() as u8;
        let target_pair = candidate_pairs[target_index as usize];
        let pair_index = name_pairs.iter().position(|pair| pair == target_pair).unwrap();
        name_pairs.remove(pair_index);
        println!("{} -> {}", giver, target_pair.1);
    }
}

fn random_numbers() -> Vec<u8> {
    let mut base = vec![123, 45, 98, 5, 101, 85, 207, 254];
    let mut result = vec![];

    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());
    result.append(&mut base.clone());

    result
}
