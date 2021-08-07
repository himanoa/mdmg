use comrak::nodes::{AstNode, NodeValue};
use crate::Result;
use crate::scaffold::Scaffold;
use comrak::{format_commonmark, parse_document, Arena, ComrakOptions};

pub fn parse(markdown: &str) -> Result<Vec<Scaffold>> {
    let arena = Arena::new();
    let doc = parse_document(&arena, &markdown, &ComrakOptions::default());

    let mut file_name: Option<String> = None;
    let mut scaffolds<Vec<Scaffold>> = vec![];

    for node in doc.children() {
        let ast = node.data.clone().into_inner().value;
        match ast {
            NodeValue::Heading(c) if c.level == 2 => {}
            _ => continue
        }

        if let NodeValue::Text(txt_vec) = node.data.clone().into_inner().value {
            if txt_vec.len() == 0 {
                continue;
            }
            let title_txt = String::from(txt_vec);
            file_name = title_txt;
        }

        if let NodeValue::CodeBlock(txt_vec) = node.data.clone().into_inner().value {
            if txt_vec.len() == 0 {
                continue;
            }
            let title_txt = String::from(txt_vec);
            file_name = title_txt;
        }
    }

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_is_return_to_one_scaffold() {
        let markdown =
r#"
## src/foobar.rs
```rust
use crate::Result;

fn something() -> Result<String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use suepr::*;

    #[test]
    fn test_something() {
    }
}
```
"#;
        let scaffolds = parse(&markdown).unwrap();
        assert_eq!(scaffolds, vec![Scaffold { file_name: "src/foobar.rs".to_string(), file_body: markdown.to_string() }])
    }
}
