pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut sum: Vec<i32> = Vec::new();
  for x in 0..(nums.len() - 1) {
    for y in (x + 1)..nums.len() {
      if (nums.get(x).unwrap() + nums.get(y).unwrap()) == target {
        sum.push(x as i32);
        sum.push(y as i32);
      }
    }
  }
  return sum;
}


#[cfg(test)]
mod tests {
  use crate::easy::two_sum::two_sum;

  #[test]
  fn it_works() {
    let mut nums:Vec<i32> = Vec::new();
    nums.push(2);
    nums.push(3);
    nums.push(6);
    nums.push(1);
    let result = two_sum(nums, 7);
    assert_eq!(2, *result.get(0).unwrap());
    assert_eq!(3, *result.get(1).unwrap());
  }
}