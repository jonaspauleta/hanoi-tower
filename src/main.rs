use hanoi_tower::solve_hanoi;

fn main() {
    let disks = 3;
    println!("Solving Tower of Hanoi with {} disks...", disks);

    let moves = solve_hanoi(disks, "A", "C", "B");

    for (i, (disk, from, to)) in moves.iter().enumerate() {
        println!("Step {}: Move disk {} from {} to {}", i + 1, disk, from, to);
    }

    println!("Solved in {} steps!", moves.len());
}
