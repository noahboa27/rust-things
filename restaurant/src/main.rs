use restaurant::hosting;
use std::collections::HashMap;
use std::fmt;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    // re-exporting example (pub use)
    hosting::add_to_waitlist();
}

fn function1() -> fmt::Result {
    Ok(())
}
fn function2() -> IoResult<()> {
    Ok(())
}
