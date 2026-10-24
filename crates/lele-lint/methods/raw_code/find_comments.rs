use crate::RawCode;
use rustc_lexer::tokenize;
use rustc_lexer::TokenKind;

use crate::common;

pub fn find_comments(code: &RawCode) -> Vec<common::CommentHit> {
    let source: &str = **code;
    let mut hits = Vec::new();
    let mut offset: usize = 0;
    for token in tokenize(source) {
        let len = token.len;
        let start = offset;
        offset = offset.saturating_add(len);
        match token.kind {
            TokenKind::LineComment => {
                let text = source
                    .get(start..offset)
                    .unwrap_or("")
                    .trim_end()
                    .to_string();
                hits.push(common::CommentHit {
                    line: RawCode::line_of(source, start),
                    kind: common::CommentKind::Line(text),
                });
            }
            TokenKind::BlockComment { .. } => {
                hits.push(common::CommentHit {
                    line: RawCode::line_of(source, start),
                    kind: common::CommentKind::Block,
                });
            }
            _ => {}
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::find_comments;
    use crate::common::CommentKind;
    use crate::RawCode;

    #[test]
    fn test_usage() {
        let source = "let x = \"http://not-a-comment\"; // real\n/* block */\nfn f() {}\n";
        let hits = find_comments(&RawCode(source));
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].line, 1);
        assert!(matches!(
            &hits[0].kind,
            CommentKind::Line(text) if text.contains("real")
        ));
        assert!(matches!(hits[1].kind, CommentKind::Block));
        assert_eq!(hits[1].line, 2);
    }

    #[test]
    fn test_usage_doc_comments() {
        let hits = find_comments(&RawCode("/// doc\nfn f() {}\n"));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].line, 1);
    }
}
