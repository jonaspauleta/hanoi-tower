use hanoi_tower::solve_hanoi;

use std::io;

fn main() {
    println!("Enter the number of disks:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let disks: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, defaulting to 3 disks.");
            3
        }
    };

    println!("Solving Tower of Hanoi with {} disks...", disks);

    let moves = solve_hanoi(disks, "A", "C", "B");

    for (i, (disk, from, to)) in moves.iter().enumerate() {
        println!("Step {}: Move disk {} from {} to {}", i + 1, disk, from, to);
    }

    println!("Solved in {} steps!", moves.len()); }
