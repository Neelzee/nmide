#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod html;

pub mod attr;

pub mod model;

pub mod map;

pub mod interface;

pub(crate) mod util {
    use anyhow::Result;
    use rstest::rstest;
    use std::ffi::{c_char, CStr, CString};

    /// TODO: Add docs
    ///
    /// # Errors
    ///
    /// This function will return an error if TODO: Add docs
    ///
    /// # Safety
    ///
    /// * The memory pointed to by `ptr` must contain a valid nul terminator at the
    ///   end of the string.
    ///
    /// * `ptr` must be [valid] for reads of bytes up to and including the nul terminator.
    ///   This means in particular:
    ///
    ///  * The entire memory range of this `CStr` must be contained within a single allocated object!
    ///
    ///  * `ptr` must be non-null even for a zero-length cstr.
    ///
    ///  * The memory referenced by the returned `CStr` must not be mutated for
    ///    the duration of lifetime `'a`.
    ///
    /// * The nul terminator must be within `isize::MAX` from `ptr`
    ///
    pub unsafe fn from_char(c: *const c_char) -> Result<String> {
        let c_str = CStr::from_ptr(c);
        let str_slice = c_str.to_string_lossy();
        Ok(str_slice.to_string())
    }

    #[rstest]
    #[case("")]
    #[case("abc")]
    #[case("æøå")]
    #[case("FOOBAR")]
    fn test_char_conversion(#[case] a: &str) -> Result<()> {
        let c_string = CString::new(a)?;
        let c_str = c_string.as_c_str();
        let ptr = c_str.as_ptr();

        let str_slice = unsafe { from_char(ptr) }?;

        assert_eq!(str_slice, a);

        Ok(())
    }
}
