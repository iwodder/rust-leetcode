use std::process::id;

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  if nums.is_empty() || (nums.len() == 1 && nums[0] == val)  {
    return 0;
  } else if nums.len() == 1 {
    return 1;
  }
  let mut idx_1 = 0;
  let mut idx_2 = nums.len() - 1;
  while idx_2 > idx_1 {
    while idx_1 < nums.len() && nums[idx_1] != val {
      idx_1 += 1;
    }
    while idx_2 > 0 && nums[idx_2] == val {
      idx_2 -= 1;
    }
    if idx_1 < idx_2 {
      let temp = nums[idx_1];
      nums[idx_1] = nums[idx_2];
      nums[idx_2] = temp;
    }
  }
  idx_1 as i32
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    let mut v = vec![3, 2, 2, 3];
    assert_eq!(2, remove_element(&mut v, 3));
  }

  #[test]
  fn test_2() {
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(5, remove_element(&mut v, 2));
  }

  #[test]
  fn test_3() {
    let mut v = vec![];
    assert_eq!(0, remove_element(&mut v, 2));
  }

  #[test]
  fn test_4() {
    let mut v = vec![1];
    assert_eq!(1, remove_element(&mut v, 2));
  }

  #[test]
  fn test_5() {
    let mut v = vec![2];
    assert_eq!(0, remove_element(&mut v, 2));
  }

  #[test]
  fn test_6() {
    let mut v = vec![2,2];
    assert_eq!(0, remove_element(&mut v, 2));
  }

  #[test]
  fn test_7() {
    let mut v = vec![2,2];
    assert_eq!(2, remove_element(&mut v, 1));
  }

  #[test]
  fn test_8() {
    let mut v = vec![2,2,3];
    assert_eq!(1, remove_element(&mut v, 2));
  }
}