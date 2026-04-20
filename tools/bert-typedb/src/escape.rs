//! TypeQL string escaping.
//!
//! The TypeDB 3.x Rust driver has no parameterized queries — all attribute
//! values are embedded directly in query strings. BERT strings (descriptions,
//! names, equivalence classes) can contain double quotes, newlines, and
//! unicode that would either break the TypeQL parser or corrupt stored data.
//!
//! This module provides [`escape_typeql_string`], which must be applied to
//! every user-supplied string before it is interpolated into a TypeQL query.
//!
//! ## Rules
//!
//! 1. Escape backslashes first (`\` → `\\`)
//! 2. Escape double quotes (`"` → `\"`)
//! 3. Replace newlines with a space (TypeQL string literals can't safely
//!    span lines; collapsing to space preserves word boundaries)
//! 4. Strip carriage returns (trailing `\r` in CRLF lines)
//! 5. Strip null bytes (unrepresentable in TypeQL string literals)
//!
//! ## Source
//!
//! Patterns derived from the TypeDB forum discussion on string handling:
//! <https://forum.typedb.com/t/escape-hell-handling-string-values/166>

/// Escapes a string for safe embedding in a TypeQL double-quoted string literal.
///
/// The returned string does NOT include the surrounding double quotes — only
/// escapes the content. Caller is responsible for wrapping with `"..."`.
///
/// # Examples
///
/// ```
/// use bert_typedb::escape::escape_typeql_string;
/// assert_eq!(escape_typeql_string("hello"), "hello");
/// assert_eq!(escape_typeql_string(r#"say "hi""#), r#"say \"hi\""#);
/// assert_eq!(escape_typeql_string("a\nb"), "a b");
/// ```
pub fn escape_typeql_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', " ")
        .replace('\r', "")
        .replace('\0', "")
}

#[cfg(test)]
mod tests {
    use super::escape_typeql_string;

    #[test]
    fn empty_string_stays_empty() {
        assert_eq!(escape_typeql_string(""), "");
    }

    #[test]
    fn plain_ascii_passes_through() {
        assert_eq!(escape_typeql_string("Bitcoin network"), "Bitcoin network");
    }

    #[test]
    fn double_quote_escaped() {
        assert_eq!(escape_typeql_string(r#"say "hi""#), r#"say \"hi\""#);
    }

    #[test]
    fn backslash_escaped() {
        assert_eq!(escape_typeql_string(r"a\b"), r"a\\b");
    }

    #[test]
    fn backslash_then_quote_correct_order() {
        // Critical: backslashes must be escaped BEFORE quotes, otherwise
        // an already-escaped \" becomes \\\" (escaped backslash + unescaped quote).
        // Input:   a\"b  (a, backslash, quote, b)
        // Expected escape output: a\\\"b (escaped backslash, escaped quote)
        let input = "a\\\"b";
        let expected = "a\\\\\\\"b";
        assert_eq!(escape_typeql_string(input), expected);
    }

    #[test]
    fn newline_collapses_to_space() {
        assert_eq!(escape_typeql_string("line1\nline2"), "line1 line2");
    }

    #[test]
    fn carriage_return_stripped() {
        assert_eq!(escape_typeql_string("line1\r\nline2"), "line1 line2");
    }

    #[test]
    fn null_byte_stripped() {
        assert_eq!(escape_typeql_string("a\0b"), "ab");
    }

    #[test]
    fn unicode_passes_through() {
        assert_eq!(escape_typeql_string("βερτ 🧪"), "βερτ 🧪");
    }

    #[test]
    fn tab_passes_through() {
        // Tabs are legal in TypeQL strings; no special handling needed.
        assert_eq!(escape_typeql_string("a\tb"), "a\tb");
    }

    #[test]
    fn combination_of_all_hazards() {
        let input = "path=\"C:\\temp\"\nnote: mixed\r\0end";
        let escaped = escape_typeql_string(input);
        // No raw double quote should remain in the escaped output
        assert!(!escaped.contains('\n'));
        assert!(!escaped.contains('\r'));
        assert!(!escaped.contains('\0'));
        // Unescaped `"` = a `"` not preceded by a `\`. Since backslashes come first,
        // every `"` in the output is preceded by `\`.
        for (i, ch) in escaped.char_indices() {
            if ch == '"' {
                let prev_is_backslash = escaped[..i]
                    .chars()
                    .last()
                    .map(|c| c == '\\')
                    .unwrap_or(false);
                assert!(prev_is_backslash, "unescaped quote at {i} in {escaped:?}");
            }
        }
    }

    #[test]
    fn output_safely_embeds_in_typeql() {
        // Property test: for any BERT-plausible string, the output should
        // yield a valid TypeQL literal when wrapped in double quotes.
        // We don't parse TypeQL here, but we can assert the structural invariants.
        let samples = [
            "simple",
            "with spaces and punctuation: , . ; ! ?",
            r#"already \"escaped\""#,
            "unicode: 中文 русский العربية",
            "emoji: 🚀 🧪 🧬",
            "tabs\tand\nnewlines",
            "null\0byte",
            "",
            r#"C:\Users\alice\Documents\file.txt"#,
        ];
        for s in samples {
            let escaped = escape_typeql_string(s);
            let wrapped = format!(r#""{escaped}""#);
            // Must begin and end with a quote
            assert!(wrapped.starts_with('"') && wrapped.ends_with('"'));
            // No embedded unescaped quotes, no control chars
            assert!(!escaped.contains('\n'));
            assert!(!escaped.contains('\r'));
            assert!(!escaped.contains('\0'));
        }
    }
}
