use std::str::FromStr;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val,
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut new_list: Box<ListNode>;
  if l1.is_some() && l2.is_some() {
    let num1 = create_num(l1);
    let num2 = create_num(l2);
    let result = num1 + num2;
    let result = String::from(result);
    result.chars().rev().for_each(||{
      new_list.
    });
    Some(new_list)
  } else {
    None
  }

}

fn create_num(ll: Option<Box<ListNode>>) -> i32 {
  let mut num = String::new();
  while ll.unwrap().next != None {
    num.push(l1.unwrap());
  }
  let num = num.chars().rev().collect();
  i32::from_str(&num).unwrap()
}