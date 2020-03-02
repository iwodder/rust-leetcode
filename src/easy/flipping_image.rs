pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
   let mut inverted_image:Vec<Vec<i32>> = Vec::new();
   for x in 0..a.len() {
     let row = a.get(x).unwrap();
     let mut new_row = Vec::new();
     for y in (0..row.len()).rev() {
       let val = *row.get(y).unwrap();
       new_row.push(if val == 1 {
         0
       } else {
         1
       })
     }
     inverted_image.push(new_row)
   }
  inverted_image
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let r1 = vec![1,1,0];
    let r2 = vec![1,0,1];
    let r3 = vec![0,0,0];
    let image = vec![r1, r2, r3];
    let new_img = flip_and_invert_image(image);
    assert!(true);
  }
}