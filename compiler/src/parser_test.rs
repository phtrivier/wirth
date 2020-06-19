#[cfg(test)]
mod test {

    use crate::parser::Parser;

    // TODO(pht) a simpler thing to test will be to pass
    // a scanner to the parser (rather than create it)
    // and call the various parse_expression() function with
    // 'chunks' of expressions.
    // This way, I can test
    #[test]
    fn test_parses_valid_expression() {
        let content = String::from("1 + 2 + 3 | 5");
        let c = Parser::parse(&content);
        assert!(c.is_ok(), format!("{:?}", c));
    }

    #[test]
    fn test_breaks_on_invalid_expression() {
        let content = String::from("@1 + x.Bar[0]");
        let c = Parser::parse(&content);
        assert!(c.is_err());
    }
}
