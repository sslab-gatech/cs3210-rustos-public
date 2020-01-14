// FIXME: Make me compile! Diff budget: 2 lines.

// I AM NOT DONE

// Do not change this definition.
enum MyEnum {
    A(String),
    B(String)
}

fn matcher(val: &MyEnum) -> &str {
    match *val {
        MyEnum::A(string) => string.as_str(),
        MyEnum::B(string) => string.as_str()
    }
}

fn main() { }
