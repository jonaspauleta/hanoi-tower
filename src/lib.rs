/// Represents a move of a disk from one peg to another.
/// Tuple contains (disk_number, from_peg, to_peg)
pub type Move = (u32, String, String);

/// Solves the Tower of Hanoi puzzle for `n` disks.
///
/// # Arguments
///
/// * `n` - The number of disks.
/// * `from` - The name of the source peg.
/// * `to` - The name of the destination peg.
/// * `aux` - The name of the auxiliary peg.
///
/// # Returns
///
/// A vector of moves, where each move is a tuple `(disk_number, from_peg, to_peg)`.
pub fn solve_hanoi(n: u32, from: &str, to: &str, aux: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    move_disks(n, from, to, aux, &mut moves);
    moves
}

/// Recursive helper function to move disks.
fn move_disks(n: u32, from: &str, to: &str, aux: &str, moves: &mut Vec<Move>) {
    if n == 1 {
        moves.push((1, from.to_string(), to.to_string()));
        return;
    }

    move_disks(n - 1, from, aux, to, moves);
    moves.push((n, from.to_string(), to.to_string()));
    move_disks(n - 1, aux, to, from, moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_hanoi_1_disk() {
        let moves = solve_hanoi(1, "A", "C", "B");
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], (1, "A".to_string(), "C".to_string()));
    }

    #[test]
    fn test_solve_hanoi_2_disks() {
        let moves = solve_hanoi(2, "A", "C", "B");
        assert_eq!(moves.len(), 3);
        assert_eq!(moves[0], (1, "A".to_string(), "B".to_string()));
        assert_eq!(moves[1], (2, "A".to_string(), "C".to_string()));
        assert_eq!(moves[2], (1, "B".to_string(), "C".to_string()));
    }

    #[test]
    fn test_solve_hanoi_3_disks() {
        let moves = solve_hanoi(3, "A", "C", "B");
        assert_eq!(moves.len(), 7);
        // Middle move should be biggest disk moving from A to C
        assert_eq!(moves[3], (3, "A".to_string(), "C".to_string()));
    }
}
