// 1068: Maze Solver — Backtracking on 2D Grid

use std::collections::VecDeque;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// Approach 1: DFS backtracking
fn solve_maze(maze: &[Vec<i32>], start: (usize, usize), end_: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut path = Vec::new();

    fn dfs(
        r: usize, c: usize, end_: (usize, usize),
        maze: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, path: &mut Vec<(usize, usize)>,
        rows: usize, cols: usize,
    ) -> bool {
        if (r, c) == end_ {
            path.push((r, c));
            return true;
        }
        if maze[r][c] == 1 || visited[r][c] { return false; }
        visited[r][c] = true;
        path.push((r, c));
        for &(dr, dc) in &DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                if dfs(nr as usize, nc as usize, end_, maze, visited, path, rows, cols) {
                    return true;
                }
            }
        }
        path.pop();
        false
    }

    if dfs(start.0, start.1, end_, maze, &mut visited, &mut path, rows, cols) {
        Some(path)
    } else {
        None
    }
}

// Approach 2: BFS for shortest path
fn solve_maze_bfs(maze: &[Vec<i32>], start: (usize, usize), end_: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut parent = vec![vec![(usize::MAX, usize::MAX); cols]; rows];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start.0][start.1] = true;

    let mut found = false;
    while let Some((r, c)) = queue.pop_front() {
        if (r, c) == end_ { found = true; break; }
        for &(dr, dc) in &DIRS {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if maze[nr][nc] == 0 && !visited[nr][nc] {
                    visited[nr][nc] = true;
                    parent[nr][nc] = (r, c);
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    if !found { return None; }
    let mut path = vec![end_];
    let (mut r, mut c) = end_;
    while (r, c) != start {
        let (pr, pc) = parent[r][c];
        path.push((pr, pc));
        r = pr;
        c = pc;
    }
    path.reverse();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_maze() -> Vec<Vec<i32>> {
        vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0],
        ]
    }

    #[test]
    fn test_dfs() {
        let maze = test_maze();
        let path = solve_maze(&maze, (0, 0), (4, 4)).unwrap();
        assert_eq!(*path.first().unwrap(), (0, 0));
        assert_eq!(*path.last().unwrap(), (4, 4));
    }

    #[test]
    fn test_bfs() {
        let maze = test_maze();
        let path = solve_maze_bfs(&maze, (0, 0), (4, 4)).unwrap();
        assert_eq!(*path.first().unwrap(), (0, 0));
        assert_eq!(*path.last().unwrap(), (4, 4));
    }

    #[test]
    fn test_impossible() {
        let maze = vec![vec![0, 1], vec![1, 0]];
        assert!(solve_maze(&maze, (0, 0), (1, 1)).is_none());
    }
}
