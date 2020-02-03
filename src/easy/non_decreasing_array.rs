pub fn check_possibility(nums: Vec<i32>) -> bool {
  if nums.len() == 0 {
    return true;
  }
  let mut cnt = 0;
  for x in 0..(nums.len() - 1) {
    if nums[x] >= nums[x + 1]  {
      cnt += 1;
    }
    if cnt > 1 {
      return false;
    }
  }
  true
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    let v = vec![4,2,3];
    assert!(check_possibility(v));
  }

  #[test]
  fn test_2() {
    let v = vec![4,2,1];
    assert!(!check_possibility(v));
  }

  #[test]
  fn test_3() {
    let v = vec![];
    assert!(check_possibility(v));
  }

  #[test]
  fn test_4() {
    let v = vec![1];
    assert!(check_possibility(v));
  }

  #[test]
  fn test_5() {
    let v = vec![0,1,2,3,1,4,5,6,7,9,20,2];
    assert!(!check_possibility(v));
  }

  #[test]
  fn test_6() {
    let v = vec![0,1,2,3,4,5,6,7,9,20,21];
    assert!(check_possibility(v));
  }

  #[test]
  fn test_7() {
    let v = vec![3,4,2,3];
    assert!(!check_possibility(v));
  }
}