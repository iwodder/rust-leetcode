pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  if nums.len() > 0 {
    let mut length = 1;
    let mut next_num = 0;
    for current_num in 0..(nums.len() - 1) {
      while next_num < nums.len() && nums[current_num] == nums[next_num] {
        next_num += 1;
      }
      if next_num < nums.len() {
        nums[current_num + 1] = nums[next_num];
        length += 1;
      } else {
        break;
      }
    }
    length
  } else {
    0
  }
}

#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn test_1() {
    let mut v = vec![1,1,2];
    assert_eq!(2, remove_duplicates(&mut v));
  }

  #[test]
  fn test_2() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!(5, remove_duplicates(&mut v));
  }

  #[test]
  fn test_3() {
    let mut v = vec![];
    assert_eq!(0, remove_duplicates(&mut v));
  }
}