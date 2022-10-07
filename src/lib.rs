use proc_macro::TokenStream;

/// A macro to change the path of `module/mod.rs` to `module/_mod.rs`
///
/// ## Example
#[cfg_attr(doctest, doc = " ````no_test")]
/// ```
/// root
/// └─┬─ main.rs
///   └─ foobar/
///      └┬─ _mod.rs
///       ├─ bar.rs
///       └─ foo.rs
/// ```
/// __main.rs:__
#[cfg_attr(doctest, doc = " ````no_test")]
/// ```rust
/// _mod!(foobar);
/// //...
/// use underscore_mod::_mod;
/// //...
/// fn main() {}
/// ```
#[proc_macro]
pub fn _mod(item: TokenStream) -> TokenStream {
	format!("#[path = \"{}/_mod.rs\"] mod {};", item, item).parse::<TokenStream>().unwrap()
}
