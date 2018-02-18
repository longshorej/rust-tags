pub fn escape_text(text: &str) -> String {
    text
        .chars()
        .map(|c| match c {
            '<'  => format!("&lt;"),
            '>'  => format!("&gt;"),
            '"'  => format!("&quot;"),
            '\'' => format!("&apos;"),
            '&'  => format!("&amp;"),
            _    => format!("{}", c),

        })
        .collect()
}

#[cfg(test)]
mod tests {
    use util::escape_text;

    #[test]
    fn escape_text_works_empty() {
        assert_eq!(escape_text(""), "")
    }

    #[test]
    fn escape_text_works_basic() {
        assert_eq!(escape_text("this is a test"), "this is a test")
    }

    #[test]
    fn escape_text_works_html() {
        assert_eq!(escape_text("<hello world /> \"123\" wow! & stuff' yeah"), "&lt;hello world /&gt; &quot;123&quot; wow! &amp; stuff&apos; yeah")
    }
}
