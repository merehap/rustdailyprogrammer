
fn main() {
    let squares = vec![
        [[8, 1, 6],
         [3, 5, 7],
         [4, 9, 2]],

         [[2, 7, 6],
          [9, 5, 1],
          [4, 3, 8]],

         [[3, 5, 7],
          [8, 1, 6],
          [4, 9, 2]],

         [[8, 1, 6],
          [7, 5, 3],
          [4, 9, 2]],
    ];

    for square in squares {
        println!("{:?}: {:?}", square, verify_magic_square(&square));
    }
}

fn verify_magic_square(square: &[[u8; 3]; 3]) -> bool {
    for row in 0..3 {
        let mut sum = 0;
        for col in 0..3 {
            sum += square[row][col];        
        }

        if sum != 15 {
            return false;
        }
    }


    for col in 0..3 {
        let mut sum = 0;
        for row in 0..3 {
            sum += square[row][col];        
        }

        if sum != 15 {
            return false;
        }
    }

    if square[0][0] + square[1][1] + square[2][2] != 15 {
        return false;
    }

    if square[2][0] + square[1][1] + square[0][2] != 15 {
        return false;
    }

    true
}
