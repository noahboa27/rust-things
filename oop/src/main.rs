use oop::blog::Post;
use oop::gui::{Button, Draw, Screen};

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }
//
// impl Draw for SelectBox {
//     fn draw(&self) {
//         // real code
//     }
// }

fn main() {
    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         }),
    //     ],
    // };
    //
    // screen.run();

    // let mut post = Post::new();
    //
    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());
    //
    // post.request_review();
    // assert_eq!("", post.content());
    //
    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
