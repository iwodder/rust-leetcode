use std::str::FromStr;

pub fn reverse(x: i32) -> i32 {
  let mut string_num = x.to_string();
  if string_num.starts_with("-") {
    string_num.remove(0);
  }
  let result = i32::from_str(&string_num.chars().rev().collect::<String>());
  if result.is_err() {
    0
  } else {
    let answer = result.unwrap();
    if string_num.starts_with("-") {
      -answer
    } else {
      answer
    }
  }
}

#[cfg(tests)]
mod tests {
  use crate::easy::reverse_integer::reverse;

  #[test]
  fn test_1() {
    assert_eq!(123, reverse(321));
  }

  #[test]
  fn test_2() {
    assert_eq!(-123, reverse(-321));
  }

  #[test]
  fn test_3() {
    assert_eq!(21, reverse( 120));
  }
}