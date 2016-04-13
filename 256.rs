
fn main() {
    let input = vec![vec![0 ,1 ,2 ,3 ,4 ,5],
                     vec![6 ,7 ,8 ,9 ,10,11],
                     vec![12,13,14,15,16,17],
                     vec![18,19,20,21,22,23],
                     vec![24,25,26,27,28,29],
                     vec![30,31,32,33,34,35]];

    let diags = oblique(&input);
    print_2_d_vector(&diags);
    println!("");
    let square = deoblique(&diags);
    print_2_d_vector(&square);
}

fn oblique(rect: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut diags = vec![];

    let mut vert_index = 0;
    for row in rect.iter() {
        let mut vert_offset = 0;
        for &elem in row {
            if diags.len() <= vert_index + vert_offset {
                diags.push(vec![elem]);
            } else {
                diags[vert_index + vert_offset].push(elem);
            }

            vert_offset += 1;
        }


        vert_index += 1;
    }

    diags
}

fn deoblique(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let square_dim = input.len() / 2 + 1;
    let mut diags = input.clone();
    let mut square = vec![vec![]; square_dim];

    for vert_index in 0..square_dim {
        for vert_offset in 0..square_dim {
            square[vert_index].push(diags[vert_index + vert_offset].remove(0));
        }
    }

    square
}

fn print_2_d_vector(vec: &Vec<Vec<u8>>) {
    for row in vec.iter() {
        for elem in row.iter() {
            print!("{} ", elem);
        }

        println!("");
    }
}
