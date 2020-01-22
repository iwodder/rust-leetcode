use std::collections::hash_set::HashSet;

const EMPTY:i32 = 0;
const FRESH:i32 = 1;
const ROTTEN: i32 = 2;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
  r: usize,
  c: usize
}
impl Point {
  fn new(r: usize, c: usize) -> Point {
    Point { r,c }
  }
}

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
  let mut fresh = HashSet::new();
  let mut rotten = Vec::new();
  let height = grid.len();
  let width = grid.get(0).as_ref().unwrap().len();
  let mut moves = 0;
  let mut new_grid = grid.clone();

  for r in 0..height {
    for c in 0..width {
      if FRESH == grid[r][c] {
        fresh.insert(Point::new(r, c));
      }
      if ROTTEN == grid[r][c] {
        rotten.push(Point::new(r, c));
      }
    }
  }
  let mut infected = rotten.len();
  while rotten.len() > 0 && fresh.len() > 0{
    let mut new_infected = 0;
    for x in 0..infected {
      let mut rot = rotten.remove(0);
      let mut p: Point;
      if (rot.r + 1) < grid.len() {
        if new_grid[rot.r + 1][rot.c] == 1 {
          new_grid[rot.r + 1][rot.c] = 2;
          p = Point::new(rot.r +1, rot.c);
          if fresh.contains(&p) {
            fresh.remove(&p);
            rotten.push(p);
            new_infected += 1;
          }
        }//right
      }
      if rot.r >= 1 {
        if new_grid[rot.r - 1][rot.c] == 1 {
          new_grid[rot.r - 1][rot.c] = 2;
          p = Point::new(rot.r -1, rot.c);
          if fresh.contains(&p) {
            fresh.remove(&p);
            rotten.push(p);
            new_infected += 1;
          }
        } //left
      }
      if (rot.c + 1) < grid[0].len() {
        if new_grid[rot.r][rot.c + 1] == 1 {
          new_grid[rot.r][rot.c + 1] = 2;
          p = Point::new(rot.r, rot.c +1);
          if fresh.contains(&p) {
            fresh.remove(&p);
            rotten.push(p);
            new_infected += 1;
          }
        }//top
      }
      if rot.c >= 1 {
        if new_grid[rot.r][rot.c - 1] == 1 {
          new_grid[rot.r][rot.c - 1] = 2;
          p = Point::new(rot.r, rot.c -1);
          if fresh.contains(&p) {
            fresh.remove(&p);
            rotten.push(p);
            new_infected += 1;
          }
        }//bottom
      }
    }
    if new_infected > 0 {moves += 1}
    infected = new_infected;
  }
  if fresh.len() > 0 {
    moves = -1;
  }
  moves
}

#[cfg(test)]
mod test {
  use super::*;
  
   #[test]
   fn test_1() {
     let v1 = vec![2,1,1];
     let v2 = vec![1,1,0];
     let v3 = vec![0,1,1];
     let v = vec![v1, v2, v3];
     assert_eq!(4, oranges_rotting(v));
   }

   #[test]
   fn test_2() {
     let v1 = vec![0,2];
     let v = vec![v1];
     assert_eq!(0, oranges_rotting(v));
   }

   #[test]
   fn test_3() {
     let v1 = vec![2,1,1];
     let v2 = vec![0,1,1];
     let v3 = vec![1,0,1];
     let v = vec![v1, v2, v3];
     assert_eq!(-1, oranges_rotting(v));
   }

   #[test]
   fn test_4() {
     let v1 = vec![2];
     let v2 = vec![1];
     let v = vec![v1, v2];
     assert_eq!(1, oranges_rotting(v));
   }

  #[test]
  fn test_5() {
    let v1 = vec![2,1,1];
    let v2 = vec![1,1,0];
    let v3 = vec![0,1,1];
    let v = vec![v1, v2, v3];
    assert_eq!(4, oranges_rotting(v));
  }
}