fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("matrix:");
    pretty_print(&matrix);

    let transpose = transpose(matrix);
    println!("transposed:");
    pretty_print(&transpose);
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for row in 0..result.len() {
        for col in 0..matrix[row].len() {
            result[row][col] = matrix[col][row];
        }
    }
    return result;
}
