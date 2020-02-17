pub fn len_longest_fib_subseq(a: Vec<i32>) -> i32 {
  let mut longest = 0;
  let mut length = 2;
  let mut first = 0;

  while first < a.len() - 2 {
    let mut num_1 = a[first];
    let mut second = first + 1;
    let temp = num_1;

    while second < a.len() - 1 {
      let mut num_2 = a[second];
      let mut idx: usize = second + 1;

      while idx < a.len() {
      let mut sum = num_1 + num_2;
        if sum == a[idx] {
          num_1 = num_2;
          num_2 = a[idx];
          length += 1;
          if longest < length {
            longest = length;
          }
        }
        idx += 1;
      }
      num_1 = temp;
      length = 2;
      second += 1;
    }
    length = 2;
    first += 1;
  }
  longest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(len_longest_fib_subseq(v), 5);
  }

  #[test]
  fn test_2() {
    let v = vec![1, 3, 7, 11, 12, 14, 18];
    assert_eq!(len_longest_fib_subseq(v), 3);
  }

  #[test]
  fn test_3() {
    let v = vec![2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50];
    assert_eq!(len_longest_fib_subseq(v), 5);
  }
}