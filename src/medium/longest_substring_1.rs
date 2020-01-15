pub fn length_of_longest_substring(s: String) -> i32 {
  let mut set:Vec<char> = Vec::new();
  let mut longest_substring:i32 = 0;
  for mut char in s.chars().into_iter() {
    char.make_ascii_lowercase();
    if set.contains(&char) {
      if set.len() as i32 > longest_substring {
        longest_substring = set.len() as i32;
      }
      while set.get(0).unwrap() != &char {
        set.remove(0);
      }
      set.remove(0);
      set.push(char);
    } else {
      set.push(char);
    }
  }
  if set.len() as i32 > longest_substring {
    longest_substring = set.len() as i32;
  }
  longest_substring
}

#[cfg(test)]
mod tests {
  use crate::medium::longest_substring_1::length_of_longest_substring;

  #[test]
  pub fn test_1() {
    let s_1 = String::from("abcabcbbb");
    assert_eq!(length_of_longest_substring(s_1), 3);
  }
  #[test]
  pub fn test_2() {
    let s_1 = String::from("bbbbb");
    assert_eq!(length_of_longest_substring(s_1), 1);
  }
  #[test]
  pub fn test_3() {
    let s_1 = String::from("pwwkew");
    assert_eq!(length_of_longest_substring(s_1), 3);
  }
  #[test]
  pub fn test_4() {
    let s_1 = String::from("aab");
    assert_eq!(length_of_longest_substring(s_1), 2);
  }
  #[test]
  pub fn test_5() {
    let s_1 = String::from("ohvhjdml");
    assert_eq!(length_of_longest_substring(s_1), 6);
  }
}