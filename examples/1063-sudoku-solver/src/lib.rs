#![allow(dead_code)]
#![allow(clippy::all)]
// 1063: Sudoku Solver — Backtracking + Constraints

// Approach 1: Simple backtracking
fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    fn is_valid(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
        for i in 0..9 {
            if board[row][i] == num || board[i][col] == num {
                return false;
            }
        }
        let (br, bc) = ((row / 3) * 3, (col / 3) * 3);
        for r in br..br + 3 {
            for c in bc..bc + 3 {
                if board[r][c] == num {
                    return false;
                }
            }
        }
        true
    }

    fn solve(board: &mut [[u8; 9]; 9]) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == 0 {
                    for num in 1..=9 {
                        if is_valid(board, r, c, num) {
                            board[r][c] = num;
                            if solve(board) {
                                return true;
                            }
                            board[r][c] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    solve(board)
}

// Approach 2: With constraint arrays for O(1) validation
fn solve_sudoku_fast(board: &mut [[u8; 9]; 9]) -> bool {
    let mut rows = [[false; 10]; 9];
    let mut cols = [[false; 10]; 9];
    let mut boxes = [[false; 10]; 9];

    for r in 0..9 {
        for c in 0..9 {
            let v = board[r][c] as usize;
            if v != 0 {
                rows[r][v] = true;
                cols[c][v] = true;
                boxes[(r / 3) * 3 + c / 3][v] = true;
            }
        }
    }

    fn solve(
        board: &mut [[u8; 9]; 9],
        rows: &mut [[bool; 10]; 9],
        cols: &mut [[bool; 10]; 9],
        boxes: &mut [[bool; 10]; 9],
    ) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == 0 {
                    let b = (r / 3) * 3 + c / 3;
                    for num in 1..=9usize {
                        if !rows[r][num] && !cols[c][num] && !boxes[b][num] {
                            board[r][c] = num as u8;
                            rows[r][num] = true;
                            cols[c][num] = true;
                            boxes[b][num] = true;
                            if solve(board, rows, cols, boxes) {
                                return true;
                            }
                            board[r][c] = 0;
                            rows[r][num] = false;
                            cols[c][num] = false;
                            boxes[b][num] = false;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    solve(board, &mut rows, &mut cols, &mut boxes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_board() -> [[u8; 9]; 9] {
        [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]
    }

    #[test]
    fn test_sudoku_simple() {
        let mut board = test_board();
        assert!(solve_sudoku(&mut board));
        assert_eq!(board[0][2], 4);
        assert_eq!(board[4][4], 5);
    }

    #[test]
    fn test_sudoku_fast() {
        let mut board = test_board();
        assert!(solve_sudoku_fast(&mut board));
        assert_eq!(board[0][2], 4);
        assert_eq!(board[4][4], 5);
    }
}
