// FIXME: Make me pass! Diff budget: 2 lines.

// I AM NOT DONE

// What traits does this struct need to derive?
#[derive(Debug)]
struct MyType(usize);

fn borrow2() {
    let mut x = MyType(1);
    let y = &x;

    // Do not modify this line.
    y.0 = 2;
    assert_eq!(*y, MyType(2));
}
