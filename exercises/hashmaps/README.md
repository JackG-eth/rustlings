# Hashmaps

A _hash map_ allows you to associate a value with a particular key.
You may also know this by the names [_unordered map_ in C++](https://en.cppreference.com/w/cpp/container/unordered_map),
[_dictionary_ in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an _associative array_ in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

- More complex topic, clone explination.

If we want to copy the variable and use it outside of the function we have to clone it to create a new string.

In the code snippet you provided, clone() is used to create new String instances for the name field of the Team struct when inserting a new team into the scores HashMap. Let's examine why clone() is necessary in this context.

When inserting a new team into the HashMap, you need to provide a value for the name field of the Team struct. The name field is of type String, which is a heap-allocated string in Rust.

In Rust, ownership rules prevent moving or copying values without explicitly indicating the intention. When you call scores.insert(team_1_name.clone(), ...), you are creating a new String instance by cloning the existing team_1_name string. This allows you to transfer ownership of the cloned string to the name field of the Team struct without invalidating or conflicting with the existing ownership of team_1_name.

The same applies to the team_2_name.clone() call when inserting the second team into the HashMap.

By using clone() in this way, you ensure that each inserted team has its own distinct String instance for the name field. This avoids potential issues with ownership and ensures that the Team struct in the HashMap has a valid and independent string for each team's name.

Note that clone() creates a new copy of the original string, which incurs some performance overhead. However, in this case, it is necessary to maintain the integrity and ownership of the String values within the Team struct and the HashMap.
