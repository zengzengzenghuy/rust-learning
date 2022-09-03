# Lesson Learned

1. You need to use `pub` for a trait, and declare in another mod using `use` same as declaring a struct in order to use it in another module.
2. To print a struct object, you have to `#[Derive(Debug)]` when defining a struct, and put `{#?}` when printing the struct.

## Bug

Fix the new_user return Box<dyn School> bug

## Reference

1. https://blog.logrocket.com/fundamentals-for-using-structs-in-rust/
