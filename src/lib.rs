//! This crate brings Slack's terrific [Block Kit 🔗] to
//! the Rust ecosystem.
//!
//! This crate should hopefully come in handy if you need to
//! build some rich functionality, or just want to send some
//! slack messages without having to know Block Kit.
//!
//! Inside, you'll find simple models with an API that is
//! thoroughly documented and (hopefully) easy to use.
//!
//! This is currently being actively developed so watch the repo for a
//! stable v1 release!
//!
//! [Block Kit 🔗]: https://api.slack.com/block-kit
//!
//! # Contributing
//!
//! If you're interested in contributing, head over to the [issues] and see what's left to
//! do to get this crate fully usable and stable - at the time of writing there are a few
//! big-picture things left to do:
//!
//! - Implement Block Elements ([#61](https://github.com/cakekindel/slack-blocks-rs/issues/61))
//! - ~~Implement Composition Objects ([#63](https://github.com/cakekindel/slack-blocks-rs/issues/63))~~
//! - Remove the `validator` crate from the public API ([#9](https://github.com/cakekindel/slack-blocks-rs/issues/9))
//! - Add a `validation` crate feature ([#8](https://github.com/cakekindel/slack-blocks-rs/issues/8))
//!
//! And this doesn't block a v1.0.0, but is definitely something I'm interested in doing for this crate,
//! that will make it a lot nicer to interact with:
//! - Add a proc-macro of some kind that allows easy creation of block messages (#??)
//!
//! This repo follows [Conventional Commits] in order to fully automate the semver process,
//! but you don't _need_ to follow this convention since the repo is configured for Squash
//! commits on merge.
//!
//! [issues]: https://github.com/cakekindel/slack-blocks-rs/issues/
//! [Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/

#[macro_use]
extern crate validator_derive;

pub mod block_elements;
pub mod blocks;
pub mod compose;
mod val_helpr;

pub use compose::text;

#[macro_export]
#[doc(hidden)]
#[deprecated]
macro_rules! impl_from_contents {
    ($enum_name:ident, $variant:ident, $contents_type:ty) => {
        impl From<$contents_type> for $enum_name {
            fn from(contents: $contents_type) -> Self {
                $enum_name::$variant(contents)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! convert {
    (impl From<$source:ty> for $dest:ty => $closure:expr) => {
        impl From<$source> for $dest {
            fn from(src: $source) -> Self {
                $closure(src)
            }
        }
    };
    (impl<$ty_var:ident> From<$source:ty> for $dest:ty => $closure:expr) => {
        impl<$ty_var> From<$source> for $dest {
            fn from(src: $source) -> Self {
                $closure(src)
            }
        }
    };
    (impl<'_> From<$source:ident> for $dest:ident => $closure:expr) => {
        impl<'a> From<$source<'a>> for $dest<'a> {
            fn from(src: $source<'a>) -> $dest<'a> {
                $closure(src)
            }
        }
    };
    (impl<'a> From<$source:ty> for $dest:ty => $closure:expr) => {
        impl<'a> From<$source> for $dest {
            fn from(src: $source) -> $dest {
                $closure(src)
            }
        }
    };
    (impl From<impl $trait_:ident<$source:ty>> for $dest:ty => $closure:expr) => {
        impl<T> From<T> for $dest
        where
            T: $trait_<$source>,
        {
            fn from(src: T) -> Self {
                $closure(src)
            }
        }
    };
    (impl<'_> From<impl $trait_:ident<$source:ident>> for $dest:ident => |$param:ident| $body:expr) => {
        impl<'a, T> From<T> for $dest<'a>
        where
            T: $trait_<$source<'a>>,
        {
            fn from($param: T) -> Self {
                $body
            }
        }
    };
}
