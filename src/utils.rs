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

pub fn jp_fixed_width_string(s: &str, w: usize) -> String {
    let num_spaces = std::cmp::max(w as isize - (s.len() / 3 * 2) as isize, 0);
    format!("{}{:<width$}", s, "", width=num_spaces as usize)
}
