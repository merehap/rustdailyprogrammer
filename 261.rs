
fn main() {
    let squares = vec![
        vec![vec![8, 1, 6],
             vec![3, 5, 7],
             vec![4, 9, 2]],

         vec![vec![2, 7, 6],
              vec![9, 5, 1],
              vec![4, 3, 8]],

         vec![vec![3, 5, 7],
              vec![8, 1, 6],
              vec![4, 9, 2]],

         vec![vec![8, 1, 6],
              vec![7, 5, 3],
              vec![4, 9, 2]],
    ];

    for square in squares {
        println!("{:?}: {:?}", square, verify_magic_square(15, &square));
    }
}

fn verify_magic_square(magic_number: u8, square: &Vec<Vec<u8>>) -> bool {
    let len = square.len();
    for row in 0..len {
        let mut sum = 0;
        for col in 0..len {
            sum += square[row][col]; 
        }

        if sum != magic_number {
            return false;
        }
    }

    for col in 0..len {
        let mut sum = 0;
        for row in 0..len {
            sum += square[row][col];        
        }

        if sum != magic_number {
            return false;
        }
    }

    let mut sum = 0;
    for index in 0..len {
        sum += square[index][index];    
    }

    if sum != magic_number {
        return false;
    }

    sum = 0;
    for index in 0..len {
        sum += square[len - index - 1][index];    
    }

    if sum != magic_number {
        return false;
    }

    true
}
