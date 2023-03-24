use n_queens::utils::{get_queens_from_input, generate_bitset_columns};

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    let queens = get_queens_from_input(n);
    let bitsets = generate_bitset_columns(n, &queens);
    
    n_queens::Solver::new(n, bitsets, queens).solve();
}
