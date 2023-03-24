pub mod utils{

    const MOVES_QUEEN: [(i8, i8); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    ///
    /// Get the positions of the queens from the input.
    /// The input is a square matrix of size n, where each row is a string of length n.
    /// Each character in the string is either 'Q' or '0'.
    /// 'Q' represents a queen, and '0' represents an empty space.
    /// The position of the queens are the positions of the Qs in the matrix.
    /// The position of a queen is represented by a tuple of the form (x, y).
    /// x is the row number, and y is the column number.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the board.
    ///
    /// # Returns
    ///
    /// * `Vec<(u8, u8)>` - A vector of tuples representing the positions of the queens.
    ///
    /// # Example
    ///
    /// ```
    /// let n = 4;
    /// let queens = n_queens::utils::get_queens_from_input(n);
    /// assert_eq!(queens, vec![(0, 2), (1, 0), (2, 3)]);
    /// ```
    ///
    /// ## Input
    ///
    /// ```text
    /// 4
    /// 00Q0
    /// Q000
    /// 000Q
    /// 0000
    /// ```
    ///
    pub fn get_queens_from_input(n: usize) -> Vec<(u8, u8)> {
        let mut queens: Vec<(u8, u8)> = Vec::new();
        let mut line = String::new();

        for i in 0..n {
            std::io::stdin().read_line(&mut line).unwrap();
            for (j, c) in line.chars().enumerate() {
                if c == 'Q' {
                    queens.push((i as u8, j as u8));
                }
            }
            line.clear();
        }
        queens
    }

    ///
    /// Generate a vector of bitsets, one for each column.
    /// Each bitset represents the positions available for a queen in that column.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the board.
    /// * `queens` - A vector of tuples representing the positions of the queens.
    ///
    /// # Returns
    ///
    /// * `Vec<u128>` - A vector of bitsets, one for each column.
    ///
    /// # Example
    ///
    /// ```
    /// let n = 4;
    /// let queens = vec![(0, 2), (1, 0), (2, 3)];
    /// let bitsets = n_queens::utils::generate_bitset_columns(n, &queens);
    /// assert_eq!(bitsets, vec![0b1111, 0b0111, 0b1111, 0b1111]);
    /// ```
    ///
    pub fn generate_bitset_columns(n: usize, queens: &Vec<(u8, u8)>) -> Vec<i128> {

        let mut bitsets: Vec<i128> = vec![0; n];

        for queen in queens.iter() {

            bitsets[queen.1 as usize] |= 1 << queen.0;

            for (row, col) in MOVES_QUEEN.iter() {
                let mut new_row = queen.0 as i8 + row;
                let mut new_col = queen.1 as i8 + col;

                while new_row >= 0 && new_row < n as i8 && new_col >= 0 && new_col < n as i8 {
                    bitsets[new_col as usize] |= 1 << new_row;
                    new_row += row;
                    new_col += col;
                }
            }
        }

        bitsets
    }

}

///
/// A struct that represents a solver for the n-queens problem.
///
/// # Attributes
///
/// * `n` - The size of the board.
/// * `bitsets` - A vector of bitsets, one for each column indicating the positions available for a queen in that column.
/// * `all_ones` - A bitset with all bits set to 1.
///
pub struct Solver{
    pub n: usize,
    pub bitsets: Vec<i128>,
    pub all_ones: i128,
    pub queens: Vec<i8>
}

impl Solver{
    ///
    /// Create a new solver for the n-queens problem.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the board.
    /// * `bitsets` - A vector of bitsets, one for each column indicating the positions available for a queen in that column.
    ///
    /// # Returns
    ///
    /// * `Solver` - A solver for the n-queens problem.
    ///
    /// # Example
    ///
    /// ```
    /// let n = 4;
    /// let queens = vec![(0, 2), (1, 0), (2, 3)];
    /// let bitsets = n_queens::utils::generate_bitset_columns(n, &queens);
    ///
    /// let solver = n_queens::Solver::new(n, bitsets, queens);
    /// assert_eq!(solver.n, 4);
    /// assert_eq!(solver.bitsets, vec![0b1111, 0b0111, 0b1111, 0b1111]);
    /// assert_eq!(solver.all_ones, 0b1111);
    /// assert_eq!(solver.queens, vec![1, -1, 0, 2]);
    /// ```
    ///
    pub fn new(n: usize, bitsets: Vec<i128>, queens_positions: Vec<(u8,u8)>) -> Solver{
        let all_ones: i128 = (1 << n) - 1;

        let mut queens = vec![-1; n];

        for queen in queens_positions.iter(){
            queens[queen.1 as usize] = queen.0 as i8;
        }

        Solver{n, bitsets, all_ones, queens}
    }

    pub fn solve(&mut self){
        self.solve_recursive(0, 0, 0, 0);
    }

    fn solve_recursive(&mut self, rows: i128, left_diagonals: i128, right_diagonals: i128, col: usize){
        if col == self.n{
            self.print_solution();
            return;
        }

        if self.queens[col] != -1{
            self.solve_recursive(
                rows,
                left_diagonals >> 1,
                right_diagonals << 1,
                col + 1
            );
            return;
        }

        let mut available = self.all_ones & !(rows | left_diagonals | right_diagonals | self.bitsets[col]);

        while available != 0 {
            let position = available & -available;
            let row = position.trailing_zeros() as usize;
            self.queens[col] = row as i8;
            self.solve_recursive(
                rows | position,
                (left_diagonals | position) >> 1,
                (right_diagonals | position) << 1,
                col + 1
            );
            self.queens[col] = -1;
            available ^= position;
        }
    }

    ///
    /// Print the solution to the n-queens problem.
    /// The queens are represented by a Q and the empty spaces by a dot.
    /// The solution is printed to the standard output.
    ///
    fn print_solution(&self) {
        for row in 0..self.n {
            for col in 0..self.n {
                if self.queens[col] == row as i8 {
                    print!("Q");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}