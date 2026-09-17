use std::ops::{Deref, DerefMut};

pub struct TestProjectProcess {
    child: std::process::Child,
}

impl TestProjectProcess {
    pub fn stdout(&mut self) -> &mut std::process::ChildStdout {
        self.child
            .stdout
            .as_mut()
            .expect("Test project process must have stdout")
    }

    pub fn stdin(&mut self) -> &mut std::process::ChildStdin {
        self.child
            .stdin
            .as_mut()
            .expect("Test project process must have stdin")
    }
}

impl Deref for TestProjectProcess {
    type Target = std::process::Child;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl DerefMut for TestProjectProcess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}
