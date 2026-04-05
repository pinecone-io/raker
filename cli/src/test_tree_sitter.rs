use tree_sitter::{Parser, Query, QueryCursor, Language};
fn test(language: Language, query_str: &str, content: &str) {
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(content, None).unwrap();
    let query = Query::new(&language, query_str).unwrap();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());
    while let Some(m) = matches.next() {}
}
