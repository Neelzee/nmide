//#[cfg(test)]
//mod rs_c;

#[cfg(test)]
mod rs_wrapper {
    use nmide_rust_ffi::html::Html;

    #[test]
    fn name() {
        extern "Rust" {
            fn foobar() -> Html;
        }
        let res = unsafe { foobar() };
        println!("{res:?}");
    }
}
