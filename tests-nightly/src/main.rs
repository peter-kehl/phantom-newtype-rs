//! Copied from `phantom-newtype`'s doc-tests. There's at least one difference between running the
//! same code in a doc test and here in a binary crate: In doc tests we didn't get `deprecated`
//! warnings.
#![feature(generic_const_exprs)]

use core::fmt;
use phantom_newtype::DisplayerOf;

enum Message {}

// This causes ICE:
//type MessageId = phantom_newtype::Id<Message, [u8; 32]>;

// No ICE:
#[allow(deprecated)]
type MessageId = phantom_newtype::IdForFlags<
    { phantom_newtype::trait_flag::TRAIT_FLAGS_NO_COPY_NO_DEFAULT },
    Message,
    [u8; 32],
>;

impl DisplayerOf<MessageId> for Message {
    fn display(id: &MessageId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        id.get().iter().try_for_each(|b| write!(f, "{:02x}", b))
    }
}

fn main() {
    let vec: Vec<_> = (0u8..32u8).collect();
    let mut arr: [u8; 32] = [0u8; 32];
    (&mut arr[..]).copy_from_slice(&vec[..]);

    assert_eq!(
        format!("{}", MessageId::from(arr).display()),
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    );
}
