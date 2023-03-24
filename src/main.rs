use n_queens::utils::{get_queens_from_input, generate_bitset_columns};

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        println!("Error reading the input.");
        std::process::exit(1);
    });

    let n: usize = input.trim().parse().unwrap_or_else(|_| {
        println!("The first line of the input must be a number.");
        std::process::exit(1);
    });

    if n < 4 || n > 128 {
        println!("The input must be a number between 4 and 128.");
        std::process::exit(1);
    }

    let queens = get_queens_from_input(n).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    let bitsets = generate_bitset_columns(n, &queens).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    println!("Find all solutions? (y/n)");

    input.clear();

    std::io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        println!("Error reading the input.");
        std::process::exit(1);
    });

    let find_all_solutions = match input.trim() {
        "y" | "Y" => true,
        _ => false
    };

    let mut solver = n_queens::Solver::new(
        n,
        bitsets,
        queens,
        !find_all_solutions
    );

    solver.solve();
    solver.print_solutions();

}
