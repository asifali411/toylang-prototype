#[derive(Debug, Clone)]
pub struct Class {
  name: String
}

impl Class {
  pub fn new(name: String) -> Self {
    Self {
      name,
    }
  }
}