//! # Menu
//!
//! A basic command-line interface for `#![no_std]` Rust programs. Peforms
//! zero heap allocation.
#![no_std]
#![feature(trait_alias)]

#[path = "."]
pub mod asynchronous {
    use bisync::asynchronous::*;
    mod menu;
    pub use menu::*;
    pub mod menu_manager;

    pub trait Read = embedded_io_async::Read + embedded_io::Read;
    pub trait Write = embedded_io_async::Write + embedded_io::Write;

    #[cfg(feature = "noline")]
    use noline::async_editor::Editor;
}

#[path = "."]
pub mod synchronous {
    use bisync::synchronous::*;
    mod menu;
    pub use menu::*;
    pub mod menu_manager;

    use embedded_io::{Read, Write};

    #[cfg(feature = "noline")]
    use noline::sync_editor::Editor;
}
