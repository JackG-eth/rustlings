# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

- you need to place it into a String if you want to use some of the mehtods.

-- interesting to note what type it is.

    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

Certainly! In Rust, both string slices (&str) and strings (String) represent textual data, but they have some fundamental differences in ownership, mutability, and memory allocation.

Ownership and Memory Allocation:

String: The String type represents an owned, mutable, and growable UTF-8 encoded string. It owns the memory used to store the string data on the heap. String objects are mutable and can be modified or resized as needed. They provide flexibility but come with the overhead of memory allocation and deallocation.
&str (String Slice): The &str type, also known as a string slice, represents an immutable view into a string data. It is a borrowed reference to a sequence of UTF-8 bytes stored elsewhere in memory, often within a String or a string literal. &str doesn't own the data it references and is therefore immutable. String slices are useful for efficiently working with existing string data without making unnecessary copies.
Mutability:

String: String objects are mutable. You can modify their content by appending, replacing, or removing characters.
&str (String Slice): &str is an immutable reference. You cannot modify the contents of a string slice directly. Instead, you would need to create a new String and assign the modified content to it.
Ownership and Borrowing:

String: String objects own the memory they use to store the string data. They can be passed around, moved, or cloned, allowing for flexible ownership transfer.
&str (String Slice): &str represents a borrowed reference to a string data. It borrows the ownership of the underlying string. String slices are lightweight and can be passed around efficiently as immutable references.
To summarize, a String is an owned, mutable, and growable string, while a &str (string slice) is an immutable view into existing string data. String offers more flexibility and allows modifications, while &str is more lightweight and efficient for working with existing string data without ownership.

It's important to note that Rust provides implicit conversions between String and &str through the concept of "Deref coercion," which makes working with them more convenient. This allows you to pass a String where a &str is expected, and vice versa, in many cases without explicitly converting them.
