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
  let mut valid = true;
  'outer: for r in 0..board.len() {
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
      if (r % 3) == 0 && (c % 3) == 0 {
        if !check_squares(&board, r, c) {
          valid = false;
          break 'outer;
        }
      }
    }
  }
  if valid {
    check_points(points)
  } else {
    valid
  }
}

fn check_squares(sub_board: &Vec<Vec<char>>, start_row:usize, start_col:usize) -> bool {
  let mut nums = [false; 10];
  let mut valid = true;
  'outer: for r in 0..3 as usize {
    for c in 0..3 as usize {
      if sub_board[start_row + r][start_col + c] != '.' {
        let x = sub_board[start_row + r][start_col + c].to_digit(10).unwrap() as usize;
        if nums[x] != false {
          valid = false;
          break 'outer;
        } else {
          nums[x] = true;
        }
      }
    }
  }
  valid
}

fn check_points(mut p: HashMap<char, Vec<Point>>) -> bool {
  let mut valid = true;
  'outer: for c in p.keys() {
    let mut rows = [false; 9];
    let mut cols = [false; 9];
    let val = p.get(c).unwrap();
    for x in 0..val.len() {
      let point = val.get(x).unwrap();
      let r = point.r;
      let c = point.c;
      if rows[r] == true || cols[c] == true {
        valid = false;
        break'outer;
      } else {
        rows[r] = true;
        cols[c] = true;
      }
    }
  }
  valid
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
  #[test]
  fn test_2() {
    let v1 = vec!['5','3','.','.','7','.','.','.','.'];
    let v2 = vec!['6','.','.','1','9','5','.','.','.'];
    let v3 = vec!['.','9','8','.','.','.','.','6','.'];
    let v4 = vec!['8','.','6','.','6','.','.','.','3'];
    let v5 = vec!['4','.','.','8','.','3','.','.','1'];
    let v6 = vec!['7','.','.','.','2','.','.','.','6'];
    let v7 = vec!['.','6','.','.','.','.','2','8','.'];
    let v8 = vec!['.','.','.','4','1','9','.','.','5'];
    let v9 = vec!['.','.','.','.','8','.','.','7','9'];
    let board = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];
    assert!(!is_valid_sudoku(board));
  }
  #[test]
  fn test_3() {
    let v1 = vec!['9','3','.','.','7','.','.','.','.'];
    let v2 = vec!['6','.','.','1','9','5','.','.','.'];
    let v3 = vec!['.','9','8','.','.','1','.','6','.'];
    let v4 = vec!['1','.','2','.','6','.','.','.','3'];
    let v5 = vec!['4','.','.','8','.','3','.','.','1'];
    let v6 = vec!['7','.','.','.','2','.','.','.','6'];
    let v7 = vec!['.','6','.','.','.','.','2','8','.'];
    let v8 = vec!['.','.','.','4','1','9','.','.','5'];
    let v9 = vec!['.','.','.','.','8','.','.','7','9'];
    let board = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];
    assert!(!is_valid_sudoku(board));
  }
  #[test]
  fn test_4() {
    let v1 = vec!['8','3','.'];
    let v2 = vec!['6','.','.'];
    let v3 = vec!['.','9','8'];
    let board = vec![v1, v2, v3];
    assert!(!check_squares(&board, 0, 0))
  }
  #[test]
  fn test_5() {
    let v1 = vec!['.','.','.','.','5','.','.','1','.'];
    let v2 = vec!['.','4','.','3','.','.','.','.','.'];
    let v3 = vec!['.','.','.','.','.','3','.','.','1'];
    let v4 = vec!['8','.','.','.','.','.','.','2','.'];
    let v5 = vec!['.','.','2','.','7','.','.','.','.'];
    let v6 = vec!['.','1','5','.','.','.','.','.','.'];
    let v7 = vec!['.','.','.','.','.','2','.','.','.'];
    let v8 = vec!['.','2','.','9','.','.','.','.','.'];
    let v9 = vec!['.','.','4','.','.','.','.','.','.'];
    let board = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];
    assert!(!is_valid_sudoku(board));
  }
}