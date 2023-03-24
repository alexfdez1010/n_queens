//! A library for solving the n-queens problem.


/// A module containing utility functions.
pub mod utils{
    use std::io;

    ///
    /// A constant array of tuples representing the moves of a queen.
    /// Each tuple represents a move of the queen in the form (row, column).
    ///
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
    /// # Errors
    ///
    /// * If the input is not a square matrix of size n.
    /// * If the input is not a matrix of characters 'Q' and '0'.
    /// * If there is an error reading the input.
    ///
    pub fn get_queens_from_input(n: usize) -> Result<Vec<(u8, u8)>, String> {
        let mut queens: Vec<(u8, u8)> = Vec::new();
        let mut line = String::new();

        for i in 0..n {
            io::stdin()
                .read_line(&mut line)
                .map_err(|_| "Error reading the input.".to_string())?;

            line = line.trim().to_string();

            if line.len() != n {
                return Err(format!("The input is not a square matrix of size {}.", n));
            }

            for (j, c) in line.chars().enumerate() {
                if c == 'Q' {
                    queens.push((i as u8, j as u8));
                    continue;
                }

                if c != '0' {
                    return Err("The input must be a matrix of characters 'Q' and '0'.".to_string());
                }
            }
            line.clear();
        }

        Ok(queens)
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
    /// # Errors
    ///
    /// * If is a invalid configuration. That is, if two queens are attacking each other.
    ///
    pub fn generate_bitset_columns(
        n: usize,
        queens: &Vec<(u8, u8)>,
    ) -> Result<Vec<i128>, String> {
        let mut bitsets: Vec<i128> = vec![0; n];

        for queen in queens.iter() {
            if bitsets[queen.1 as usize] & (1 << queen.0) != 0 {
                return Err("Invalid configuration. There are two queens attacking each other.".to_string());
            }

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

        Ok(bitsets)
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
/// * `queens` - A vector of integers representing the positions of the queens.
/// * `solutions` - A vector of vectors of integers representing the positions of the queens in each solution.
/// * `solved` - A boolean indicating whether the solver has been called.
/// * `only_one_solution` - A boolean indicating whether the solver should only find one solution.
/// * `solution_found` - A boolean indicating whether a solution has been found.
///
pub struct Solver{
    pub n: usize,
    pub queens: Vec<i8>,
    pub solutions: Vec<Vec<i8>>,
    pub only_one_solution: bool,
    bitsets: Vec<i128>,
    all_ones: i128,
    solved: bool,
    solution_found: bool,
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
    pub fn new(n: usize, bitsets: Vec<i128>,
               queens_positions: Vec<(u8,u8)>,
               only_one_solution: bool) -> Solver {

        let all_ones: i128 = (1 << n) - 1;

        let mut queens = vec![-1; n];

        for queen in queens_positions.iter(){
            queens[queen.1 as usize] = queen.0 as i8;
        }

        Solver{n, bitsets, all_ones,
            queens, solutions: Vec::new(),
            solved: false, only_one_solution, solution_found: false}
    }

    ///
    /// Solve the n-queens problem according to the configuration of the solver.
    /// The solutions are stored in the `solutions` attribute.
    ///

    pub fn solve(&mut self){
        self.solve_recursive(0, 0, 0, 0);
        self.solved = true;
    }

    fn solve_recursive(&mut self, rows: i128, left_diagonals: i128, right_diagonals: i128, col: usize){
        if self.solution_found && self.only_one_solution{
            return;
        }

        if col == self.n{
            self.solutions.push(self.queens.clone());
            self.solution_found = true;
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
    /// # Arguments
    ///
    /// * `solution` - A vector of integers representing the position of the queens in each column.
    ///
    fn print_solution(&self, solution: &Vec<i8>) {

        for row in 0..self.n {

            for col in 0..self.n {
                let cell = if solution[col] == row as i8 { "♛" } else { "·" };
                print!(" {cell}");
            }
            println!();
        }

        println!();
    }

    ///
    /// Print all the solutions to the n-queens problem.
    /// The queens are represented by a Q and the empty spaces by a dot.
    /// The solutions are printed to the standard output.
    /// If the solver has not been called, it will be called to get the solutions.
    ///
    pub fn print_solutions(&mut self){
        if !self.solved{
            self.solve();
        }

        if self.solutions.len() == 0{
            println!("There are no solutions for the configuration provided.");
            return;
        }

        for solution in self.solutions.iter(){
            self.print_solution(solution);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use super::Solver;

    #[test]
    fn test_generate_bitset_columns() {
        let n = 4;
        let queens = vec![(0, 2), (1, 0), (2, 3)];
        let bitsets = generate_bitset_columns(n, &queens).unwrap();
        assert_eq!(bitsets, vec![0b1111, 0b0111, 0b1111, 0b1111]);
    }

    #[test]
    fn test_solve() {
        let n = 4;
        let queens = vec![(0, 2), (1, 0), (2, 3)];
        let bitsets = generate_bitset_columns(n, &queens).unwrap();
        let mut solver = Solver::new(n, bitsets, queens, false);
        solver.solve();
        assert_eq!(solver.solutions.len(), 1);
        assert_eq!(solver.solutions[0], vec![1, 3, 0, 2]);

        let n = 5;
        let queens = vec![(0, 0)];
        let bitsets = generate_bitset_columns(n, &queens).unwrap();
        let mut solver = Solver::new(n, bitsets, queens, false);
        solver.solve();
        assert_eq!(solver.solutions.len(), 2);
        assert_eq!(solver.solutions[0], vec![0, 2, 4, 1, 3]);
        assert_eq!(solver.solutions[1], vec![0, 3, 1, 4, 2]);

        let n = 5;
        let queens = vec![(0, 0)];
        let bitsets = generate_bitset_columns(n, &queens).unwrap();
        let mut solver = Solver::new(n, bitsets, queens, true);
        solver.solve();
        assert_eq!(solver.solutions.len(), 1);
        assert_eq!(solver.solutions[0], vec![0, 2, 4, 1, 3]);
    }
}