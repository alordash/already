pub use code_reload_proc_macro::hotreload;

#[doc(hidden)]
pub use code_reload_core::LibraryWrapper;

#[cfg(feature = "runtime")]
pub mod runtime {
    pub use code_reload_build::runtime::{
        HotreloadLibrary, IHotreloadPayload, LockedHotreloadLibrary, start_watch,
    };
    pub use code_reload_build::start_watchers;
    pub use code_reload_build::{add_runtime, add_tests_runtime};
    pub use code_reload_build::{build, build_dir, build_tests};
}

use mockall_double::double;
mod thing {
    use mockall::automock;
    pub struct Thing{}
    #[automock]
    impl Thing {
        pub fn work(&self) {
            self.extra_work();
        }
        
        pub fn extra_work(&self) {}
    }
}

#[double]
use thing::Thing;

#[cfg(test)]
mod t {
    use mockall::Sequence;
    use super::*;

    #[test]
    fn test_foo() {
        let mut seq = Sequence::new();
        let mut mock = Thing::default();
        
        mock.expect_work().times(1).in_sequence(&mut seq).returning(|| ());
        mock.expect_extra_work().times(1).in_sequence(&mut seq).returning(|| ());
        
        mock.work();
    }
}