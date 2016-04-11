
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
    let mut top_diag_sum = 0;
    let mut bot_diag_sum = 0;
    for i in 0..square.len() {
        let mut row_sum = 0;
        let mut col_sum = 0;
        for j in 0..square.len() {
            row_sum += square[i][j]; 
            col_sum += square[j][i];
        }

        if col_sum != magic_number || row_sum != magic_number {
            return false;
        }

        top_diag_sum += square[i][i];    
        bot_diag_sum += square[square.len() - i - 1][i];    
    }

    top_diag_sum == magic_number && bot_diag_sum == magic_number
}
