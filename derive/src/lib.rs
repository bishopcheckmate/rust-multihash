//! This proc macro derives a custom Multihash code table from a list of hashers.
//!
//! The digests are stack allocated with a fixed size. That size needs to be big enough to hold any
//! of the specified hash digests. This cannot be determined reliably on compile-time, hence it
//! needs to set manually via the `alloc_size` attribute. Also you might want to set it to bigger
//! sizes then necessarily needed for backwards/forward compatibility.
//!
//! If you set `#mh(alloc_size = …)` to a too low value, you will get compiler errors. Please note
//! the the sizes are checked only on a syntactic level and *not* on the type level. This means
//! that digest need to have a size generic, which is a valid `typenum`, for example `U32` or
//! `generic_array::typenum::U64`.
//!
//! You can disable those compiler errors with setting the `no_alloc_size_errors` attribute. This
//! can be useful if you e.g. have specified type aliases for your hash digests and you are sure
//! you use the correct value for `alloc_size`.
//!
//! # Example
//!
//! ```
//! use tiny_multihash::derive::Multihash;
//! use tiny_multihash::{U32, U64, MultihashCode};
//!
//! #[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
//! #[mh(alloc_size = U64)]
//! pub enum Code {
//!     #[mh(code = 0x01, hasher = tiny_multihash::Sha2_256, digest = tiny_multihash::Sha2Digest<U32>)]
//!     Foo,
//!     #[mh(code = 0x02, hasher = tiny_multihash::Sha2_512, digest = tiny_multihash::Sha2Digest<U64>)]
//!     Bar,
//! }
//!
//! let hash = Code::Foo.digest(b"hello world!");
//! println!("{:02x?}", hash);
//! ```
extern crate proc_macro;

mod multihash;
mod utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use synstructure::{decl_derive, Structure};

decl_derive!([Multihash, attributes(mh)] => #[proc_macro_error] multihash);
fn multihash(s: Structure) -> TokenStream {
    multihash::multihash(s).into()
}
