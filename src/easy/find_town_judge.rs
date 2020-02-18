use std::collections::HashMap;

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
  if n == 1 { return 1; }
  let mut trust_map:HashMap<i32, Vec<i32>> = HashMap::new();
  for x in 1..=n {trust_map.insert(x, Vec::new());}
  trust.iter().for_each(|pair|{
      trust_map.get_mut(&pair[0]).unwrap().push(pair[1]);
  });
  let mut person = -1;
  for (k,v) in trust_map {
    if v.len() == 0 {
      if person == -1 {
        person = k;
      } else {
        person = -1;
        break;
      }
    }
  }
  person
}

#[cfg(test)]
mod tests {
  use crate::easy::find_town_judge::find_judge;

  #[test]
  fn test_1() {
    let v = vec![1,2];
    let v_1 = vec![v];
    assert_eq!(find_judge(2, v_1), 2);
  }

  #[test]
  fn test_2() {
    let v_1 = vec![1,3];
    let v_2 = vec![2,3];
    let v = vec![v_1,v_2];
    assert_eq!(find_judge(3, v), 3);
  }

  #[test]
  fn test_3() {
    let v_1 = vec![1,3];
    let v_2 = vec![2,3];
    let v_3 = vec![3,1];
    let v = vec![v_1,v_2,v_3];
    assert_eq!(find_judge(3, v), -1);
  }

  #[test]
  fn test_4() {
    let v_1 = vec![1,2];
    let v_2 = vec![2,3];
    let v = vec![v_1,v_2];
    assert_eq!(find_judge(3, v), -1);
  }

  #[test]
  fn test_5() {
    let v_1 = vec![1,3];
    let v_2 = vec![1,4];
    let v_3 = vec![2,3];
    let v_4 = vec![2,4];
    let v_5 = vec![4,3];
    let v = vec![v_1,v_2,v_3, v_4, v_5];
    assert_eq!(find_judge(4, v), 3);
  }

  #[test]
  fn test_6() {
    let v_1 = vec![1,2];
    let v_2 = vec![6,4];
    let v_3 = vec![3,2];
    let v_4 = vec![2,6];
    let v_5 = vec![4,5];
    let v_6 = vec![6,1];
    let v_7 = vec![1,4];
    let v_8 = vec![1,5];
    let v_9 = vec![2,3];
    let v_10 = vec![2,1];
    let v_11 = vec![4,3];
    let v_12 = vec![4,2];
    let v_13 = vec![2,5];
    let v_14 = vec![4,1];
    let v_15 = vec![2,4];
    let v_16 = vec![6,5];
    let v_17 = vec![3,5];
    let v = vec![v_1,v_2,v_3, v_4,v_5,
                              v_6,v_7,v_8,v_9,v_10,v_11,
                              v_12,v_13,v_14,v_15,v_16,
                              v_17];
    assert_eq!(find_judge(6, v), 5);
  }
}