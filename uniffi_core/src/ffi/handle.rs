/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Object handle
///
/// Handles opaque `u64` values used to pass objects across the FFI, both for objects implemented in
/// Rust and ones implemented in the foreign language.
///
/// Rust handles are generated by leaking a raw pointer
/// Foreign handles are generated with a handle map that only generates odd values.
/// For all currently supported architectures and hopefully any ones we add in the future:
/// * 0 is an invalid value.
/// * The lowest bit will always be set for foreign handles and never set for Rust ones (since the
///   leaked pointer will be aligned to `size_of::<Arc<T>>()` == `size_of::<*const T>()`).
///
/// Rust handles are mainly managed is through the [crate::HandleAlloc] trait.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Handle(u64);

impl Handle {
    pub fn from_pointer<T>(ptr: *const T) -> Self {
        Self(ptr as u64)
    }

    pub fn as_pointer<T>(&self) -> *const T {
        self.0 as *const T
    }

    pub fn from_raw(raw: u64) -> Option<Self> {
        if raw == 0 {
            None
        } else {
            Some(Self(raw))
        }
    }

    pub fn from_raw_unchecked(raw: u64) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> u64 {
        self.0
    }
}
