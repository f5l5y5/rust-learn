//! # hello_api
//!
//! `hello_api` is a collection of utilities to make performing certain calculations more convenient

/// Add one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = hello_api::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}
