use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T> {
  id: UnsafeCell<usize>,
  inner: UnsafeCell<T>,
  lock: AtomicBool,
}

struct Example {
  data: UnsafeCell<i32>,
}

impl Example {
  fn new(value: i32) -> Self {
    Example {
      data: UnsafeCell::new(value),
    }
  }

  fn get(&self) -> i32 {
    unsafe { *self.data.get() }
  }

  fn set(&self, value: i32) {
    unsafe {
      *self.data.get() = value; // UnsafeCell 내부 데이터 변경
    }
  }
}
struct SafeExample {
  data: i32,
}

impl SafeExample {
  fn new(value: i32) -> Self {
    SafeExample { data: value }
  }

  fn get(&self) -> i32 {
    self.data
  }

  fn set(&mut self, value: i32) {
    self.data = value;
  }
}
