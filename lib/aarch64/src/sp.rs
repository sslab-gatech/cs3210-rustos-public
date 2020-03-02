pub struct _SP;
impl _SP {
    /// Returns the current stack pointer.
    #[inline(always)]
    pub fn get(&self) -> usize {
        let rtn: usize;
        unsafe {
            asm!("mov $0, sp": "=r"(rtn) ::: "volatile");
        }
        rtn
    }

    /// Set the current stack pointer with an passed argument.
    #[inline(always)]
    pub unsafe fn set(&self, stack: usize) {
        asm!("mov sp, $0":: "r"(stack) :: "volatile");
    }
}
pub static SP: _SP = _SP {};
