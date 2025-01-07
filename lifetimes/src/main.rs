use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");

    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }
}

// lifetimes are the relationship between the parameters and the return type
// a contract much like generics
//
// this is restricting the function signature to say that the return value has
// to have an equal lifetime to the shortest lifetime of the parameters
// from the scope it is from
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32             // a reference
// &'a i32          // a reference with an explicit lifetime
// &'a mut i32      // a mutable reference with an explicit lifetime

// This is the longest function from Listing 10-21 that returns the longer
// of two string slices. But now it has an extra parameter named ann of the
// generic type T, which can be filled in by any type that implements the
// Display trait as specified by the where clause. This extra parameter will
// be printed using {}, which is why the Display trait bound is necessary.
// Because lifetimes are a type of generic, the declarations of the lifetime
// parameter 'a and the generic type parameter T go in the same list inside
// the angle brackets after the function name.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
