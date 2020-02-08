pub fn count_digit_one(n: i32) -> i32 {
  let mut count:i64 = 0;
  let mut x:i64 = 1;
  let n = n as i64;
  while x <= n {
    let a = n / x;
    let b = n % x;
    if a % 10 > 1 {
      count += a/10 * x + x;
    } else if a%10 == 1 {
      count += a/10 * x + b + 1;
    } else {
      count += a/10 * x;
    }
    x *= 10;
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

  #[test]
  fn test_3() {
    assert_eq!(count_digit_one(1410065408), 1737167499)
  }

  #[test]
  fn test_4() {
    assert_eq!(count_digit_one(-1), 0)
  }
}