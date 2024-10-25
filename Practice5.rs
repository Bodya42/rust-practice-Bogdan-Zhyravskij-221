const SIZE: usize = 20;  // Adjusted size to match the pattern

fn main() {
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Draw the edges of the square and the diagonal cross inside
            if i == 0 || i == SIZE - 1 || j == 0 || j == SIZE - 1 {
                print!("*"); // Border of the square
            } else if i == j || i + j == SIZE - 1 {
                print!("*"); // Diagonals
            } else {
                print!(" "); // Space inside the square
            }
        }
        println!(); // Move to the next line
    }
}
