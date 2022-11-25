pub mod nll_todo {
    /// Satisfies type checker without breaking non lexical lifetimes
    /// Does so by unwrapping an option instead of implicitly panicking.
    /// This forces rustc to consider subsequent control flows instead of forcing an immediate
    /// return.
    /// # Panics
    /// Always panics.
    #[deprecated]
    pub fn nll_todo<S>() -> S {
        None.unwrap()
    }
}
