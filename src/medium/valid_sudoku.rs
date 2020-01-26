use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct Point {
  r: usize,
  c: usize
}
impl Point {
  fn new(r: usize, c: usize) -> Point{
    Point{ r, c}
  }
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  let mut points: HashMap<char, Vec<Point>> = HashMap::new();
  for r in 0..board.len() {
    for c in 0..board[r].len() {
      if board[r][c] != '.' {
        match points.get_mut(&board[r][c]) {
          Some(x) => {
            x.push(Point::new(r, c));
          }
          None => {
            points.insert(board[r][c], vec![Point::new(r, c)]);
          }
        }
      }
    }
  }
  println!("{:?}", points);
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let v1 = vec!['5','3','.','.','7','.','.','.','.'];
    let v2 = vec!['6','.','.','1','9','5','.','.','.'];
    let v3 = vec!['.','9','8','.','.','.','.','6','.'];
    let v4 = vec!['8','.','.','.','6','.','.','.','3'];
    let v5 = vec!['4','.','.','8','.','3','.','.','1'];
    let v6 = vec!['7','.','.','.','2','.','.','.','6'];
    let v7 = vec!['.','6','.','.','.','.','2','8','.'];
    let v8 = vec!['.','.','.','4','1','9','.','.','5'];
    let v9 = vec!['.','.','.','.','8','.','.','7','9'];
    let board = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];
    assert!(is_valid_sudoku(board));
  }
}