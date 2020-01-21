use std::collections::hash_set::HashSet;

const EMPTY:i32 = 0;
const FRESH:i32 = 1;
const ROTTEN: i32 = 2;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
  x: usize, 
  y: usize
}
impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point {x, y}
  }
}

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
  let mut fresh = HashSet::new();
  let mut rotten = Vec::new();
  let height = grid.len();
  let width = grid.get(0).as_ref().unwrap().len();
  let mut moves = 0;
  let mut new_grid = grid.clone();

  for x in 0..height {
    for y in 0..width {
      if FRESH == grid[x][y] {
        fresh.insert(Point::new(x, y));
      }
      if ROTTEN == grid[x][y] {
        rotten.push(Point::new(x, y));
      }
    }
  }
  
  while rotten.len() > 0 && fresh.len() > 0{
    let mut pushed = false;
    let mut r = rotten.remove(0);
    let mut p: Point;
    if (r.x + 1) < grid.len() {
      if new_grid[r.x + 1][r.y] == 1 {
        p = Point::new(r.x+1, r.y);
        if fresh.contains(&p) {
          fresh.remove(&p);
          rotten.push(p);
          pushed = true;
        }
      }//right
    }
    if r.x >= 1 {
      if grid[r.x - 1][r.y] == 1 {
        p = Point::new(r.x-1, r.y);
        if fresh.contains(&p) {
          fresh.remove(&p);
          rotten.push(p);
          pushed = true;
        }
      } //left
    }
    if (r.y + 1) < grid[0].len() {
      if grid[r.x][r.y + 1] == 1 {
        p = Point::new(r.x, r.y+1);
        if fresh.contains(&p) {
          fresh.remove(&p);
          rotten.push(p);
          pushed = true;
        }
      }//top
    }
    if r.y >= 1 {
      if grid[r.x][r.y - 1] == 1 {
        p = Point::new(r.x,r.y-1);
        if fresh.contains(&p) {
          fresh.remove(&p);
          rotten.push(p);
          pushed = true;
        }
      }//bottom
    }
    if pushed {moves += 1}
  }
  if fresh.len() > 0 {
    moves = -1;
  }
  moves
}

#[cfg(test)]
mod test {
  use super::*;
  
  // #[test]
  // fn test_1() {
  //   let v1 = vec![2,1,1];
  //   let v2 = vec![1,1,0];
  //   let v3 = vec![0,1,1];
  //   let v = vec![v1, v2, v3];
  //   assert_eq!(4, oranges_rotting(v));
  // }

  // #[test]
  // fn test_2() {
  //   let v1 = vec![0,2];
  //   let v = vec![v1];
  //   assert_eq!(0, oranges_rotting(v));
  // }

  // #[test]
  // fn test_3() {
  //   let v1 = vec![2,1,1];
  //   let v2 = vec![0,1,1];
  //   let v3 = vec![1,0,1];
  //   let v = vec![v1, v2, v3];
  //   assert_eq!(-1, oranges_rotting(v));
  // }

  // #[test]
  // fn test_4() {
  //   let v1 = vec![2];
  //   let v2 = vec![1];
  //   let v = vec![v1, v2];
  //   assert_eq!(1, oranges_rotting(v));
  // }

  #[test]
  fn test_5() {
    let v1 = vec![2,1,1];
    let v2 = vec![1,1,0];
    let v3 = vec![0,1,1];
    let v = vec![v1, v2, v3];
    assert_eq!(4, oranges_rotting(v));
  }
}