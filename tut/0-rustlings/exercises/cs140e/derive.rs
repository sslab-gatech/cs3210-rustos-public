// FIXME: Make me compile! Diff budget: 1 line.

// I AM NOT DONE

enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

pub fn main() {
    println!("Duration: {:?}", Duration::MilliSeconds(1200));

    let x = Duration::Minutes(10);
    let y = x;
    let z = x;
}
