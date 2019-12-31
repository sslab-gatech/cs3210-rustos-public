// FIXME: Make me compile! Diff budget: 2 lines.

// I AM NOT DONE

// Do not modify the inner type &'a T.
struct RefWrapper<T>(&'a T);

// Do not modify the inner type &'b RefWrapper<'a, T>.
struct RefWrapperWrapper<T>(&'b RefWrapper<'a, T>);

pub fn main() { }
