// MIT License
//
// Copyright (c) Lukas Lueg <lukas.lueg@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Access all of the variables available from the built crate at build time.
//!
//! This crate uses the env var `BUILT_AT_BUILD` to trigger a rebuild and will not automatically
//! rebuild in any other scenario.
//!
//! ## What does this mean?
//!
//! If you build this crate for the very first time at exactly 00:00 on Oct 27th 2025 UTC with the `chrono` feature
//! enabled, the resulting built variable will look like this:
//!
//! ```
//! pub static BUILT_TIME_UTC: &str = "Mon, 27 Oct 2025 00:00:00 +0000";
//!
//! ```
//!
//! If you build again 5 minutes later this variable will not change.
//!
//! ## Why?
//! The short answer is that `cargo` has no way of knowing if the source of one of those variables
//! has changed and will always optimize for build times by assuming the crate **DOES NOT** need to
//! be rebuilt.
//!
//! The long answer can be found in [The Cargo Book](<https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection>)
//!
//! ## How to make cargo rebuild this crate every time?
//! In your `build.rs`
//! ```
//! fn main(){
//!   ensure_rebuild()
//! }
//! ```

use std::{
    hash::{DefaultHasher, Hash as _, Hasher as _},
    time::SystemTime,
};
include!(concat!(env!("OUT_DIR"), "/built_at_build.rs"));

/// Creates a best effort "unique" env var value to ensure this crate is rebuilt every time.
pub fn ensure_rebuild() {
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);

    let hash_str = format!("{}", hasher.finish());
    std::env::set_var("BUILT_AT_BUILD", hash_str);
}
