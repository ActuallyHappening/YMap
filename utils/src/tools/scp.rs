use crate::cmds::IntoArgs;
use crate::paths::RemoteFile;
use crate::prelude::*;

pub struct ScpArgs {
  pub ssh_name: String,
  pub from: RemoteFile,
  pub to_local_file: Utf8PathBuf,
}

impl IntoArgs for ScpArgs {
  fn binary_name() -> String {
    "scp".into()
  }
  fn into_args(self) -> Vec<String> {
    vec![
      format!("{}:{}", self.ssh_name, self.from),
      self.to_local_file.to_string(),
    ]
  }
}
