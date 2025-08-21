mod html_file_parsing_test {

    #[test]
    fn should_parse_file1() {
        let html = crate::translator::from_html::parse_html(include_str!("test1.html"));
        assert!(html.is_ok(), "Not ok: {:?}", html.unwrap_err());
    }
}