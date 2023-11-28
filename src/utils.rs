use skim::prelude::*;

pub struct NameItem {
    pub name: &'static str,
    pub search_text: String,
}

impl SkimItem for NameItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.search_text)
    }

    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::from(self.name)
    }

    fn output(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name)
    }
}

/// ひらがな・カタカナとasciiだけで構成された文字列の幅を返す
/// それ以外の文字列を渡した場合の結果は保証しない
pub fn jp_width(s: &str) -> usize {
    let mut w = 0;

    for ch in s.chars() {
        let v = ch as usize;
        if (0x3040..=0x309f).contains(&v) || (0x30a0..=0x30ff).contains(&v) {
            w += 2;
        } else {
            w += 1;
        }
    }

    w
}

pub fn jp_fixed_width_string(s: &str, w: usize) -> String {
    let num_spaces = std::cmp::max(w as isize - jp_width(s) as isize, 0);
    format!("{}{:<width$}", s, "", width=num_spaces as usize)
}
