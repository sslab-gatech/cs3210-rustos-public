// FIXME: Make me compile! Diff budget: 2 lines.

// I AM NOT DONE

struct MyStruct(usize);

impl MyStruct {
    fn make_1(&self) {
        self.0 = 1;
    }
}

pub fn main() {
    let x = MyStruct(10);
    x.make_1();
}
