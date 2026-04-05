use std::path::Path;
use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIterator};

pub struct ExtractedSymbols {
    pub imports: Vec<String>,
    pub definitions: Vec<String>,
}

pub fn extract_symbols(path: &Path, content: &str) -> Option<ExtractedSymbols> {
    let extension = path.extension()?.to_str()?;

    let (language, query_str) = match extension {
        "rs" => (
            tree_sitter_rust::LANGUAGE.into(),
            r#"
            (use_declaration) @import
            (function_item name: (identifier) @def)
            (struct_item name: (type_identifier) @def)
            (enum_item name: (type_identifier) @def)
            (trait_item name: (type_identifier) @def)
            "#,
        ),
        "js" | "jsx" => (
            tree_sitter_javascript::LANGUAGE.into(),
            r#"
            (import_statement) @import
            (function_declaration name: (identifier) @def)
            (class_declaration name: (identifier) @def)
            "#,
        ),
        "ts" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            r#"
            (import_statement) @import
            (function_declaration name: (identifier) @def)
            (class_declaration name: (type_identifier) @def)
            (interface_declaration name: (type_identifier) @def)
            "#,
        ),
        "tsx" => (
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            r#"
            (import_statement) @import
            (function_declaration name: (identifier) @def)
            (class_declaration name: (type_identifier) @def)
            (interface_declaration name: (type_identifier) @def)
            "#,
        ),
        "py" => (
            tree_sitter_python::LANGUAGE.into(),
            r#"
            (import_statement) @import
            (import_from_statement) @import
            (function_definition name: (identifier) @def)
            (class_definition name: (identifier) @def)
            "#,
        ),
        "go" => (
            tree_sitter_go::LANGUAGE.into(),
            r#"
            (import_declaration) @import
            (function_declaration name: (identifier) @def)
            (method_declaration name: (field_identifier) @def)
            (type_declaration (type_spec name: (type_identifier) @def))
            "#,
        ),
        _ => return None,
    };

    parse_and_query(language, query_str, content)
}

fn parse_and_query(language: Language, query_str: &str, content: &str) -> Option<ExtractedSymbols> {
    let mut parser = Parser::new();
    parser.set_language(&language).ok()?;

    let tree = parser.parse(content, None)?;
    let query = Query::new(&language, query_str).ok()?;
    let mut cursor = QueryCursor::new();

    let mut imports = Vec::new();
    let mut definitions = Vec::new();

    let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let text = capture
                .node
                .utf8_text(content.as_bytes())
                .unwrap_or("")
                .trim()
                .to_string();

            if text.is_empty() {
                continue;
            }

            match *capture_name {
                "import" => {
                    // Extract a clean representation, mostly just taking the first line to keep it brief
                    let clean_text = text.lines().next().unwrap_or(&text).to_string();
                    if !imports.contains(&clean_text) {
                        imports.push(clean_text);
                    }
                }
                "def" => {
                    if !definitions.contains(&text) {
                        definitions.push(text);
                    }
                }
                _ => {}
            }
        }
    }

    Some(ExtractedSymbols {
        imports,
        definitions,
    })
}
