pub fn longest_palindrome(s: String) -> String {
  let mut longest = String::new();
  let mut v = s.chars().collect::<Vec<char>>();
  let mut first_pos = 0;
  let mut second_pos = 1;
  while first_pos < v.len() {
    if (v.len() - first_pos) > longest.len() {
      while second_pos < v.len() + 1 {
        if is_palindrome(&v[first_pos..second_pos]) {
          if v[first_pos..second_pos].len() > longest.len() {
            longest = v[first_pos..second_pos].iter().collect::<String>();
          }
        }
        second_pos = second_pos + 1;
      }
    } else {
      break;
    }
    first_pos = first_pos + 1;
    second_pos = first_pos + 1;
  }
  longest
}


fn is_palindrome(v: &[char]) -> bool {
  let mut i = 0;
  let mut j = v.len() - 1;
  while i < j{
    if v[i] != v[j] {
      return false
    }
    i = i + 1;
    if j > 0 {
      j = j - 1;
    }
  }
  true
}

#[cfg(test)]
mod test {
  use crate::medium::longest_palindromic_substring::{longest_palindrome, is_palindrome};

  #[test]
  fn test_1() {
    assert_eq!("baaaaaaab", longest_palindrome(String::from("abcbabaaaaaaab")));
  }

  #[test]
  fn test_2() {
    assert_eq!("bbb", longest_palindrome(String::from("cbbb")));
  }

  #[test]
  fn test_is_palindrome_1() {
    let c = ['b','b'];
    assert!(is_palindrome(&c));
  }

  #[test]
  fn test_is_palindrome_2() {
    let c = ['c','b','b','b','c'];
    assert!(is_palindrome(&c));
  }

  #[test]
  fn test_is_palindrome_3() {
    let c = ['b','a','c'];
    assert!(!is_palindrome(&c));
  }
}

