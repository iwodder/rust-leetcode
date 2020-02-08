pub fn count_digit_one(n: i32) -> i32 {
  let mut count = 0;
  for x in 1..(n+1) {
    x.to_string().chars().for_each(|c|{
      if c == '1' {
        count += 1;
      }
    })
  }
  count as i32
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(count_digit_one(13),6)
  }

  #[test]
  fn test_2() {
    assert_eq!(count_digit_one(20),12)
  }
}