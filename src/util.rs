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
