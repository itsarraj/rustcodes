struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // let user1 = User {
    //     active: true,
    //     username: String::from("animesh080"),
    //     email: String::from("sadasdsad@gmsadad.com"),
    //     sign_in_count: 1,
    // };
    // let mut x = user1;
    // x.username = String::from("adas");

    let mut user1 = User {
        active: true,
        username: String::from("animesh080"),
        email: String::from("sadasdsad@gmsadad.com"),
        sign_in_count: 1,
    };
    user1.username = String::from("adas");

    // let user2 = User {
    //     active: user1.active,
    //     username: String::from("sdasesh080"),
    //     email: String::from("sasadsaddasdsad@gmsadad.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2.

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // error[E0382]: use of moved value: `user1.username`
    //   --> src/main.rs:40:17
    //    |
    // 33 |       let user2 = User {
    //    |  _________________-
    // 34 | |         email: String::from("another@example.com"),
    // 35 | |         ..user1
    // 36 | |     };
    //    | |_____- value moved here
    // ...
    // 40 |       let user3 = User {
    //    |  _________________^
    // 41 | |         email: String::from("another@example.com"),
    // 42 | |         ..user1
    // 43 | |     };
    //    | |_____^ value used here after move
    //    |
    //    = note: move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
    //
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // can use field init shorthand like username
        email: email,       // can use field init shorthand like email
        sign_in_count: 1,
    }
}
fn build_user_init_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username, // can use field init shorthand like username
        email,    // can use field init shorthand like email
        sign_in_count: 1,
    }
}
