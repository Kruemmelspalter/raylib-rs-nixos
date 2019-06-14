/* raylib-rs
   lib.rs - Main library code (the safe layer)

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

//! # raylib-rs
//!
//! `raylib` is a safe Rust binding to [Raylib](https://www.raylib.com/), a C library for enjoying games programming.
//!
//! To get started, take a look at the [`init_window`] function. This initializes Raylib and shows a window, and returns a [`RaylibHandle`]. This handle is very important, because it is the way in which one accesses the vast majority of Raylib's functionality. This means that it must not go out of scope until the game is ready to exit.
//!
//! For more control over the game window, the [`init`] function will return a [`RaylibBuilder`] which allows for tweaking various settings such as VSync, anti-aliasing, fullscreen, and so on. Calling [`RaylibBuilder::build`] will then provide a [`RaylibHandle`].
//!
//! Some useful constants can be found in the [`consts`] module, which is also re-exported in the [`prelude`] module. In most cases you will probably want to `use raylib::prelude::*;` to make your experience more smooth.
//!
//! [`init_window`]: fn.init_window.html
//! [`init`]: fn.init.html
//! [`RaylibHandle`]: struct.RaylibHandle.html
//! [`RaylibBuilder`]: struct.RaylibBuilder.html
//! [`RaylibBuilder::build`]: struct.RaylibBuilder.html#method.build
//! [`consts`]: consts/index.html
//! [`prelude`]: prelude/index.html
//!
//! # Examples
//!
//! The classic "Hello, world":
//!
//! ```
//! use raylib::prelude::*;
//!
//! fn main() {
//!     
//! }
//! ```
#![feature(optin_builtin_traits)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![allow(dead_code)]
#![doc(
    html_logo_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust_256x256.png",
    html_favicon_url = "https://github.com/deltaphc/raylib-rs/raw/master/logo/raylib-rust.ico"
)]
#![feature(test)]
extern crate test;

#[cfg(test)]
#[macro_use]
mod tests;

pub mod consts;
pub mod core;
pub mod ease;
pub mod prelude;
mod raymath;
mod safe_funcs;
mod safe_types;

/// The raw, unsafe FFI binding, in case you need that escape hatch or the safe layer doesn't provide something you need.
pub mod ffi {
    pub use raylib_sys::*;
}

pub use crate::core::*;

// pub use crate::raymath::*;
// pub use crate::safe_funcs::*;
// pub use crate::safe_types::*;
