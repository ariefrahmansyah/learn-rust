use std::collections::HashMap;

fn main() {
    // Create an empty hash map by using the HashMap::new method and then adding elements with the HashMap::insert method.
    let mut book_reviews: HashMap<String, String> = HashMap::new();

    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // After we've populated our hash map, we can query it:
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // Searching for an existing key returns the value associated to it
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // We can remove entries from a hash map by using the .remove() method:
    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);
}
