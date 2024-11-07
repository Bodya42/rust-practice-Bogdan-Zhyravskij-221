Chapter11.1


fn test1() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(&s); // Borrow `s` by passing a reference

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: &String) { // Change parameter to borrow the String
    println!("ownership of \"{}\" is moved here!", s);
}



fn test2() {  
   let mut s = String::from("hello, world");

   let slice1: &str = &s; // In two ways: either &s or &s[..]
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[..5]; // Slice the first 5 characters
   assert_eq!(slice2, "hello");

   let slice3: &mut String = &mut s; // We need a mutable reference to modify
   slice3.push('!'); // Modify the original String
   assert_eq!(slice3, "hello, world!");

   println!("Success!");
}



fn test3() {  
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!");

    // Create a slice pointing to String `s`
    let slice: &str = &s;

    // Create a String type based on the recently created slice
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}



fn test4() {
    let s = String::from("hello, 世界");
    
    // `s[0..1]` correctly gets the first character "h"
    let slice1 = &s[0..1]; // `h` takes 1 byte in UTF-8 format
    assert_eq!(slice1, "h");

    // We need to work with characters directly, not byte indices.
    let slice2 = &s[7..10]; // This is a valid slice for the character "世"
    assert_eq!(slice2, "世");

    // Iterate through all chars in s using `char_indices`
    for (i, c) in s.char_indices() { // Use `char_indices` to get char positions
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!");
}



fn test5() {
    let mut s = String::new();
    s.push_str("hello"); // Fill in: Initialize `s` with the string "hello"

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111]; // Represents the bytes for "hello"

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap(); // Fill in: Convert the byte vector into a String
    
    assert_eq!(s, s1); // Assert that both strings are equal

    println!("Success!");
}



fn test6() {
    let mut s = String::with_capacity(25); // Set the capacity to 25 to avoid reallocation

    println!("{}", s.capacity()); // This will print the initial capacity: 25

    for _ in 0..2 {
        s.push_str("hello"); // Adds 5 characters each time, without triggering reallocation
        println!("{}", s.capacity()); // Capacity should remain 25 after each push
    }

    println!("Success!");
}



use std::mem;

fn test7() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_ptr(); // Get a pointer to the string's data
    let len = story.len();    // Get the length of the string
    let capacity = story.capacity(); // Get the capacity of the string

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr as *mut u8, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}
