// A struct with named fields
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

// A tuple struct
struct Point2D(u32, u32);

// A unit struct
struct Unit;

fn main() {
    // Instantiate a classic struct, with named fields. Order does not matter.
    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Likes oranges: {}", person.likes_oranges);

    // Instantiate a tuple struct by passing the values in the same order as defined.
    let origin = Point2D(0, 0);
    println!("Origin: {}, {}", origin.0, origin.1);

    // Instantiate a unit struct.
    let unit = Unit;
}
