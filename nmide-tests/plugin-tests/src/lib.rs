#[cfg(test)]
mod framework_tests {
    use std::path::PathBuf;

    use anyhow::Result;
    use libloading::{Library, Symbol};
    use nmide_rust_ffi::{html::Html, model::Model};

    #[test]
    fn test_framework_view() -> Result<()> {
        let path = PathBuf::from(
            "../../nmide-plugin/nmide-framework/target/release/libnmide_framework.so",
        )
        .canonicalize()?;
        let lib = unsafe { Library::new(path) }
            .or_else(|_| unsafe { Library::new("libnmide_framework.so") });

        assert!(lib.is_ok());

        let lib = lib?;

        let func: Symbol<unsafe extern "Rust" fn(Model) -> Html> = unsafe { lib.get(b"view") }?;

        let res = unsafe { func(Model::new()) };

        assert_eq!("Hello, World!", res.get_text());

        Ok(())
    }
}
