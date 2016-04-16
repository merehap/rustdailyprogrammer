
fn main() {
    for (n, operation) in simple_game_of_threes(31337357) {
        let op_text = match operation {
            Operation::Divide => "0",
            Operation::Increment => "1",
            Operation::Decrement => "-1",
            Operation::Complete => "",
        };

        println!("{} {}", n, op_text);
    }
}

fn simple_game_of_threes(mut n: u32) -> Vec<(u32, Operation)> {
    let mut pairs = vec![];

    while n != 1 {
        if n % 3 == 0 {
            pairs.push((n, Operation::Divide));
            n /= 3;
        } else {
            pairs.push((n, Operation::Increment));
            n += 1;
        }
    }

    pairs.push((1, Operation::Complete));

    pairs
}

enum Operation {
    Divide,
    Increment,
    Decrement,
    Complete,
}
