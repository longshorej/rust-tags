use core::*;
use util::*;

pub fn nl2br(value: &str) -> Fragment {
    let text = escape_text(value);
    let fixed = text.replace("\r\n", "\n").replace("\n", "<br/>");

    raw(&fixed)
}
