use std::cmp::Ordering::*;
use std::io;
use std::io::BufRead;

use self::Abundance::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect::<Vec<_>>();

    println!("");

    for line in lines {
        let number = line.unwrap().trim().parse::<u32>().unwrap();
        let status_text = match classify_abundance(number) {
            Deficient(n) => format!("deficient by {}", n),
            Perfect => "perfect".to_owned(),
            Abundant(n) => format!("abundant by {}", n),
        };

        println!("{} {}", number, status_text);
    }
}

fn classify_abundance(n: u32) -> Abundance {
    let proper_factor_sum = proper_factors(n).iter().fold(0, |sum, f| sum + f);
    match n.cmp(&proper_factor_sum) {
        Less => Abundant(proper_factor_sum - n),
        Equal => Perfect,
        Greater => Deficient(n - proper_factor_sum),
    }
}

fn proper_factors(n: u32) -> Vec<u32> {
    let mut factors = vec![1];
    
    let max_factor = (n as f32).sqrt().round() as u32;
    for factor in 2..max_factor {
        if n % factor == 0 {
            factors.push(factor);
            factors.push(n / factor);
        }
    }

    if max_factor * max_factor == n {
        factors.push(max_factor);
    }

    factors
}

enum Abundance {
    Deficient(u32),
    Perfect,
    Abundant(u32),
}
