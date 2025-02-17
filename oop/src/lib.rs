pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // real code
        }
    }
}

pub mod blog {
    // Example of how the state pattern can be implemented {{{
    // pub struct Post {
    //     state: Option<Box<dyn State>>,
    //     content: String,
    // }
    //
    // impl Post {
    //     pub fn new() -> Post {
    //         Post {
    //             state: Some(Box::new(Draft {})),
    //             content: String::new(),
    //         }
    //     }
    //
    //     pub fn add_text(&mut self, text: &str) {
    //         self.content.push_str(text);
    //     }
    //
    //     pub fn content(&self) -> &str {
    //         self.state.as_ref().unwrap().content(self)
    //     }
    //
    //     pub fn request_review(&mut self) {
    //         if let Some(s) = self.state.take() {
    //             self.state = Some(s.request_review())
    //         }
    //     }
    //
    //     pub fn approve(&mut self) {
    //         if let Some(s) = self.state.take() {
    //             self.state = Some(s.approve())
    //         }
    //     }
    // }
    //
    // trait State {
    //     fn request_review(self: Box<Self>) -> Box<dyn State>;
    //     fn approve(self: Box<Self>) -> Box<dyn State>;
    //     fn content<'a>(&self, post: &'a Post) -> &'a str {
    //         ""
    //     }
    // }
    //
    // struct Draft {}
    //
    // impl State for Draft {
    //     fn request_review(self: Box<Self>) -> Box<dyn State> {
    //         Box::new(PendingReview {})
    //     }
    //
    //     fn approve(self: Box<Self>) -> Box<dyn State> {
    //         self
    //     }
    // }
    //
    // struct PendingReview {}
    //
    // impl State for PendingReview {
    //     fn request_review(self: Box<Self>) -> Box<dyn State> {
    //         self
    //     }
    //
    //     fn approve(self: Box<Self>) -> Box<dyn State> {
    //         Box::new(Published {})
    //     }
    // }
    //
    // struct Published {}
    //
    // impl State for Published {
    //     fn request_review(self: Box<Self>) -> Box<dyn State> {
    //         self
    //     }
    //
    //     fn approve(self: Box<Self>) -> Box<dyn State> {
    //         self
    //     }
    //
    //     fn content<'a>(&self, post: &'a Post) -> &'a str {
    //         &post.content
    //     }
    // }
    // }}}

    // more rustic way of doing things
    // i actually prefer this over the above solution
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
