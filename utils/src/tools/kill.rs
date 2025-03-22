use crate::cmds::NuCommand;

#[derive(Clone, Debug)]
pub struct Kill {
  pub process_names: Vec<String>,
}

impl Kill {
  pub fn names(&self) -> String {
    self.process_names.join(" ")
  }
}

impl NuCommand for Kill {
  fn command(self) -> String {
    format!(
      "ps | find {} | each {{ |proc| print $proc.name; kill $proc.pid }}",
      self.names()
    )
  }
}
