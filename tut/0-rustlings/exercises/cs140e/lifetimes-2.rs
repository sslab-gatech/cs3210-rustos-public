// FIXME: Make me compile! Diff budget: 3 lines.

// I AM NOT DONE

// Do not modify the inner type &'a T.
struct RefWrapper<T>(&'a T);

impl RefWrapper {
    fn inner(&self) -> &T {
        self.0
    }
}

// Do not modify this function.
pub fn main() {
    let x = 1;
    let mut r = &x;
    r = RefWrapper(r).inner();
}
