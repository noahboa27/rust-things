fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let mut user1 = User {
        active: true,
        username: String::from("somesuername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // since username was moved to user2, user1 is no longer valid
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // other fields stay the same
    };
}
