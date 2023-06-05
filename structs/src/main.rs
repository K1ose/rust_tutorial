struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    user_instance();
    build_user(String::from("hi"), String::from("hello"));
    build_user_shorthand(String::from("flora"), String::from("hello"));
    build_user_with_another_instance();
}

fn user_instance() {
    println!("user_instance()");
    let mut user1 = User {
        active: true,
        username: String::from("K1ose"),
        email: String::from("klose@jk404.cn"),
        sign_in_count: 1,
    };

    // if the entire instance(not only certain fields)is mutable, we can modify a value by using the dot notation and assigning into a particular field -> 'user1.email'
    user1.email = String::from("klose@ts365.cc");
}

/// build_user() return a User struct
fn build_user(username: String, email: String) -> User {
    println!("build_user()");

    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/// Use field init shorthand to avoid repetition of username and email
fn build_user_shorthand(username: String, email: String) -> User {
    println!("build_user_shorthand()");

    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

/// use the same values from user1 that we created to create user2, user3.
fn build_user_with_another_instance() {
    println!("build_user_with_another_instance()");
    let user1 = User {
        active: true,
        username: String::from("K1ose"),
        email: String::from("klose@jk404.cn"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("klose@ts365.cc"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user2.email);

    // use ..<instance name>. It must come last to specify that the remaining fields not explicityly set should have the same value as the fields in the given instance.
    /* NOTE that the struct update syntax uses = like an assignment, which means that the user1.username is invalid in user1. If you only use user1.active and user1.sign_in_count, thanks to the Copy trait, it is valid. */
    // let user3 = User {
    //     email: String::from("klose@jk4u.website"),
    //     ..user1
    // };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/// Note that the black and origin values are different types, even though their fields within the struct have the same types.
fn build_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/// Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
struct Unit_like;

fn build_unit_like_struct() {
    let subject = Unit_like;
}

// if you try to store a reference in a struct without specifying lifetimes, it won't work.
/* struct No_Sepcifying_Lifetime {
    flag: bool,
    query: &str,    // like this
}

fn try_to_use_reference_field_without_lifetime() {
    let NSL = No_Sepcifying_Lifetime {
        flag: true,
        query: "ok",
    };
} */
