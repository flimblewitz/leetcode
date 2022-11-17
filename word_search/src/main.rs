fn main() {
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".into()
        )
    );
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".into()
        )
    );
    println!(
        "{:?}",
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".into()
        )
    );
}
struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut already_used = vec![vec![false; board[0].len()]; board.len()];
        // iterate over board to find all starting positions
        // for each possible starting position, return true if the recursive function finds a solution
        // recursively_solve(0,0,&mut already_used, &board, &word.chars().collect())
        let chars: Vec<char> = word.chars().collect();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if chars[0] == board[r][c] {
                    already_used[r][c] = true;
                    if recursively_solve(r, c, &mut already_used, &board, &chars[1..]) {
                        return true;
                    }
                    already_used[r][c] = false;
                }
            }
        }
        false
    }
}

fn recursively_solve(
    r: usize,
    c: usize,
    already_used: &mut Vec<Vec<bool>>,
    board: &Vec<Vec<char>>,
    chars: &[char],
) -> bool {
    if chars.is_empty() {
        return true;
    }

    if r > 0 && !already_used[r - 1][c] && board[r - 1][c] == chars[0] {
        already_used[r - 1][c] = true;
        if recursively_solve(r - 1, c, already_used, &board, &chars[1..]) {
            return true;
        }
        already_used[r - 1][c] = false;
    }
    if r < board.len() - 1 && !already_used[r + 1][c] && board[r + 1][c] == chars[0] {
        already_used[r + 1][c] = true;
        if recursively_solve(r + 1, c, already_used, &board, &chars[1..]) {
            return true;
        }
        already_used[r + 1][c] = false;
    }
    if c > 0 && !already_used[r][c - 1] && board[r][c - 1] == chars[0] {
        already_used[r][c - 1] = true;
        if recursively_solve(r, c - 1, already_used, &board, &chars[1..]) {
            return true;
        }
        already_used[r][c - 1] = false;
    }
    if c < board[0].len() - 1 && !already_used[r][c + 1] && board[r][c + 1] == chars[0] {
        already_used[r][c + 1] = true;
        if recursively_solve(r, c + 1, already_used, &board, &chars[1..]) {
            return true;
        }
        already_used[r][c + 1] = false;
    }

    false
}
