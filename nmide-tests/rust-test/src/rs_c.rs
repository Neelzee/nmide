/// This module tests if Rust can call C functions, using the Rust-Wrapper
#[cfg(test)]
mod rs_c_test {
    use anyhow::Result;
    use rstest::rstest;
    use std::ffi::{CStr, CString};

    const EXPECTED_GREETING: &str = "Nmide-Lib sends greetings from C!";

    #[rstest]
    #[case("", "")]
    #[case("123", "123")]
    #[case("æøå", "æøå")]
    #[case("FooBAR", "FooBAR")]
    fn test_char(#[case] a: &str, #[case] b: &str) -> Result<()> {
        //let res = from_char(a);
        //assert_eq!(res, b);
        Ok(())
    }

    #[test]
    fn test_greetins() -> Result<()> {
        #[link(name = "nmide")]
        extern "C" {
            pub fn greetings() -> *mut i8;
        }

        let r = unsafe { greetings() };
        todo!();
        let res = "";
        assert_eq!(res, EXPECTED_GREETING);
        Ok(())
    }

    #[test]
    fn test_rs_greetins() -> Result<()> {
        #[link(name = "nmide_rust_ffi")]
        extern "C" {
            pub fn greetings() -> *mut i8;
        }

        let r = unsafe { greetings() };
        todo!();
        let res = "";
        assert_eq!(res, EXPECTED_GREETING);
        Ok(())
    }
}
