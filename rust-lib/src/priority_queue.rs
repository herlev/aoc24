use std::collections::BinaryHeap;

pub enum OrderBy {
  Min,
  Max,
}

#[derive(PartialEq, Eq, Debug)]
struct QueueElement<T> {
  priority: usize,
  data: T,
}

impl<T: Eq> Ord for QueueElement<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    // TODO, add support for OrderBy
    // right now it orders by Min
    other.priority.cmp(&self.priority)
  }
}

impl<T: Eq> PartialOrd for QueueElement<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug)]
pub struct PriorityQueue<T> {
  data: BinaryHeap<QueueElement<T>>,
}

impl<T: Eq> PriorityQueue<T> {
  pub fn new() -> Self {
    Self {
      data: BinaryHeap::new(),
    }
  }
  pub fn len(&self) -> usize {
    self.data.len()
  }
  pub fn push(&mut self, (priority, e): (usize, T)) {
    self.data.push(QueueElement { priority, data: e })
  }
  pub fn pop(&mut self) -> Option<T> {
    if let Some(qe) = self.data.pop() {
      return Some(qe.data);
    }
    None
  }
}

// impl<T> PriorityQueue<T> {
//   fn new(order: OrderBy) -> Self {
//     PriorityQueue {
//       data: BinaryHeap::new(),
//       order,
//     }
//   }
// }
