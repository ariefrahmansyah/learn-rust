fn main() {
    // `mascot` is not valid and cannot be used here, because it's not yet declared.
    {
        let mascot = String::from("ferris");   // `mascot` is valid from this point forward.
        // do stuff with `mascot`.
    }
    // this scope is now over, so `mascot` is no longer valid and cannot be used.

    // If we try to use mascot beyond its scope, we'll get an error like this:
    // error[E0425]: cannot find value `mascot` in this scope
    // println!("{}", mascot);

    // transfer ownership
    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        let ferris = mascot;
        // ferris dropped here. The string data memory will be freed here.

        // We cannot use mascot after we've moved ownership of the string data from mascot to ferris.
        // error[E0382]: borrow of moved value: `mascot`
        // println!("{}", mascot)
    }

    caller();
}

// ownership in functions
fn process_string(input: String) {}

fn process_u32(input: u32) {}

fn caller() {
    let s = String::from("Hello, world!");
    process_string(s); // Ownership of the string in `s` moved into `process_string`

    // The value s was moved, we cannot use it anymore
    // error[E0382]: use of moved value: `s`
    // move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // process_string(s); // Error! ownership already moved.

    // copying instead of moving
    let n = 1u32;
    process_u32(n); // Ownership of the number in `n` copied into `process_u32`
    process_u32(n); // `n` can be used again because it wasn't moved, it was copied.

    // copying types that don't implement `Copy`
    let s2 = String::from("Hello, world!");
    process_string(s2.clone()); // Passing another value, cloned from `s`.
    process_string(s2); // s was never moved and so it can still be used.
}
