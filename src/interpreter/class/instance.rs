use crate::interpreter::class::class::Class;

#[derive(Debug, Clone)]
pub struct Instance {
  class: Class,
}

impl Instance {
  pub fn new(class: Class) -> Self {
    Self {
      class,
    }
  }
}