use std::str::FromStr;
use std::borrow::Borrow;

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
  if l1.is_some() && l2.is_some() {
    let num1 = create_num(l1);
    let num2 = create_num(l2);
    let result = add(num1, num2);
    create_new_list(result)
  } else {
    None
  }
}

fn create_num(ll: Option<Box<ListNode>>) -> Vec<i32> {
  let mut num = Vec::new();
  let mut node = ll.unwrap();
  while node.next != None {
    num.push(node.val);
    node = node.next.unwrap();
  }
  num.push(node.val);
  num.reverse();
  num
}

fn create_new_list(val: String) -> Option<Box<ListNode>> {
  let mut chars = val.chars();
  let mut list = ListNode::new(chars.next().unwrap().to_digit(10).unwrap() as i32);
  let mut node = &mut list;
  for x in 1..val.len() {
    node.next = Some(Box::new(ListNode::new(chars.next().unwrap().to_digit(10).unwrap() as i32)));
    node = node.next.as_mut().unwrap().as_mut();;
  }
  Some(Box::new(list))
}

fn add(num1: Vec<i32>, num2: Vec<i32>) -> String {
  let mut result = String::new();
  let mut top;
  let mut bottom;
  if num1.len() > num2.len() {
    top = num1;
    bottom = num2;
  } else {
    top = num2;
    bottom = num1;
  }
  top.reverse();
  bottom.reverse();
  let mut carry = 0;
  for x in 0..top.len() {
    let mut dig_1 = *top.get(x).unwrap();
    let mut dig_2 = if x < bottom.len() {*bottom.get(x).unwrap()} else { 0 };
    if (dig_1 + dig_2 + carry) < 10 {
      let dig = dig_1 + dig_2 + carry;
      result.push_str(&dig.to_string());
      carry = 0;
    } else {
      let dig = (dig_1 + dig_2 + carry) % 10;
      result.push_str(&dig.to_string());
      carry = 1;
    }
  }
  if carry == 1 {
    result.push_str(&carry.to_string());
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::medium::add_two_numbers;
  use crate::medium::add_two_numbers::{ListNode, create_new_list, add, add_two_numbers};
  use std::borrow::{BorrowMut, Borrow};

  #[test]
  fn it_works() {
    let mut list_1 = ListNode::new(2);
    let mut node = Some(Box::new(ListNode::new(4)));
    let mut temp_node = node.as_mut().unwrap();
    let node_2 = Some(Box::new(ListNode::new(3)));
    temp_node.next = node_2;
    list_1.next = node;

    let mut list_2 = ListNode::new(5);
    let mut node = Some(Box::new(ListNode::new(6)));
    let mut temp_node = node.as_mut().unwrap();
    let node_2 = Some(Box::new(ListNode::new(4)));
    temp_node.next = node_2;
    list_2.next = node;

    add_two_numbers(Some(Box::new(list_1)), Some(Box::new(list_2)));
  }

  #[test]
  fn can_create_a_list() {
    let number = String::from("1999999999");
    let number_2 = String::from("9");
    let mut list = create_new_list(number).unwrap();
    assert_eq!(list.val, 1);
    let mut node = list.next.as_ref().unwrap();
    assert_eq!(node.val, 9);

  }

  #[test]
  fn can_add() {
    let mut num1 = Vec::new();
    num1.push(1);
    num1.push(2);
    num1.push(3);
    let mut num2 = Vec::new();
    num2.push(4);
    num2.push(5);
    num2.push(6);
    assert_eq!(add(num1, num2), "985");
  }
  #[test]
  fn can_add_different_length_nums() {
    let mut num1 = Vec::new();
    num1.push(1);
    let mut num2 = Vec::new();
    num2.push(4);
    num2.push(5);
    num2.push(6);
    assert_eq!(add(num1, num2), "754");
  }
  #[test]
  fn can_add_one_length_nums() {
    let mut num1 = Vec::new();
    num1.push(5);
    let mut num2 = Vec::new();
    num2.push(5);
    assert_eq!(add(num1, num2), "01");
  }
  #[test]
  fn can_add_test_length_nums() {
    let mut num1 = Vec::new();
    num1.push(3);
    num1.push(4);
    num1.push(2);
    let mut num2 = Vec::new();
    num2.push(4);
    num2.push(6);
    num2.push(5);
    assert_eq!(add(num1, num2), "708");
  }
}