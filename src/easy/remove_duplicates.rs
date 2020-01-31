pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let mut insert_pos = 1;
  for current_num in 0..(nums.len() - 1) {
    insert_pos = current_num + 1;
    let mut next_num = current_num + 1;
    while nums[current_num] == nums[next_num] {
      next_num += 1;
    }
    nums[insert_pos] = nums[next_num];
  }
  2
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
}