mod test_main;
pub use test_main::test_kernel_main;

//====================================================================
// STRUCTURES
//====================================================================

#[derive(Debug)]
pub struct __UnitTest {
    func: fn(),
    name: &'static str,
}

//====================================================================
// IMPLEMENTATIONS
//====================================================================

#[allow(unused)]
impl __UnitTest {
    pub const fn new(func: fn(), name: &'static str) -> Self {
        Self { func, name }
    }
}

//====================================================================
// FUNCTIONS
//====================================================================

