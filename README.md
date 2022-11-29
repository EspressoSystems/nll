# What is this?

The `todo!` macro panics and as a result can lose some type checking information. This crate exports module containing one function, `nll_todo`. Instead of panicking, this function unwraps an option. Nll stands for non lexical lifetime, and forces rustc to do more thorough type checking and therefore produce more informative errors. We have a blog post coming soon on this topic.

# Example usage

Use this macro to replace any occurrences of `todo!()`


```rust

pub fn example() -> u32 {
  nll_todo()
}

```


