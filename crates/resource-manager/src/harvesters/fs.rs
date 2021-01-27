use crate::harvester::Collectable;

pub struct FsHarvester {
  entry_point: String,
}

impl FsHarvester {
  pub fn new(entry_point: &str) -> FsHarvester {
    FsHarvester {
      entry_point: String::from(entry_point),
    }
  }

  pub fn entry_point(&self) -> &str {
    &self.entry_point
  }

  pub fn set_entry_point(&mut self, entry_point: &str) -> &FsHarvester {
    self.entry_point = String::from(entry_point);
    self
  }
}

impl Collectable for FsHarvester {
  fn collect(&self) {
    unimplemented!("Please implement FsHarvester.collect!!!");
  }

  fn base(&self) -> &str {
    &self.entry_point
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_fs_harvester_by_new() {
      let fsh = FsHarvester::new(".");

      assert_eq!(".", fsh.entry_point());
  }

  #[test]
  fn setter_should_change_entry_point() {
      let mut fsh = FsHarvester::new(".");
      fsh.set_entry_point("new test");

      assert_eq!("new test", fsh.entry_point());
  }
}