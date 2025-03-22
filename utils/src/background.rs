use crate::prelude::*;

type Ret = Result<()>;

#[derive(Default)]
pub struct JoinHandles(Vec<JoinHandle>);

impl JoinHandles {
  pub fn new() -> Self {
    Self::default()
  }

  /// Runs in background until manually stopped
  pub fn spawn<F>(&mut self, callback: F)
  where
    F: FnOnce() -> Ret + Send + Sync + 'static,
  {
    let handle = std::thread::spawn(callback);
    self.0.push(JoinHandle(handle));
  }

  pub fn join_all(self) -> Result<()> {
    for handle in self.0 {
      handle.0.join().expect("Thread not to panic")?;
    }

    Ok(())
  }
}

pub struct JoinHandle(std::thread::JoinHandle<Ret>);
