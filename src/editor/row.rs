use editor::syntax::{Flag, Keyword, Syntax};
use util;

use std::iter;
use std::rc::Rc;

const KILO_TAB_STOP: usize = 8;

pub struct Row {
    pub contents: String,
    pub render: String,
    pub highlight: Vec<Highlight>,
    syntax: Option<Rc<Syntax>>,
    pub open_comment: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Highlight {
    Normal,
    Comment,
    MLComment,
    Keyword1,
    Keyword2,
    String,
    Number,
    Match,
}

impl Highlight {
    pub fn to_color(&self) -> u8 {
        match *self {
            Highlight::Normal => 37,
            Highlight::Comment | Highlight::MLComment => 36,
            Highlight::Keyword1 => 33,
            Highlight::Keyword2 => 32,
            Highlight::String => 35,
            Highlight::Number => 31,
            Highlight::Match  => 34,
        }
    }

    pub fn from_keyword(kw: &Keyword) -> Self {
        match kw {
            &Keyword::One(_) => Highlight::Keyword1,
            &Keyword::Two(_) => Highlight::Keyword2,
        }
    }
}

enum InString {
    SingleQuoted,
    DoubleQuoted,
}

impl InString {
    fn to_char(&self) -> char {
        match self {
            &InString::SingleQuoted => '\'',
            &InString::DoubleQuoted => '"',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '\'' => Some(InString::SingleQuoted),
            '"' => Some(InString::DoubleQuoted),
            _ => None,
        }
    }
}

fn is_separator(c: char) -> bool {
    c.is_whitespace() ||
        c == '\0' ||
        ",.()+-/*=~%<>[];".contains(c)
}

impl Row {
    pub fn new() -> Self {
        Row {
            contents: String::new(),
            render: String::new(),
            highlight: Vec::new(),
            syntax: None,
            open_comment: false,
        }
    }

    pub fn from_string(s: String) -> Self {
        let mut row = Self::new();
        row.append_string(&s);
        row
    }

    pub fn set_syntax(&mut self, syntax: Option<Rc<Syntax>>) {
        self.syntax = syntax;
        self.update_syntax();
    }

    fn update(&mut self) {
        self.update_render();
        self.update_syntax();
    }

    pub fn starts_mlcomment(&self) -> bool {
        match self.highlight.last() {
            None => false,
            Some(hl) => {
                if hl != &Highlight::MLComment { return false }
                let mce = self.syntax.as_ref().unwrap().multiline_comment_end;
                if self.render.ends_with(mce) { return false }
                let mcs = self.syntax.as_ref().unwrap().multiline_comment_start;
                self.highlight[0] != Highlight::MLComment ||
                    self.render.starts_with(mcs)
            },
        }
    }

    pub fn ends_mlcomment(&self) -> bool {
        match self.highlight.last() {
            None => false,
            Some(hl) => {
                if self.highlight[0] != Highlight::MLComment { return false }
                let mce = self.syntax.as_ref().unwrap().multiline_comment_end;
                hl != &Highlight::MLComment ||
                    self.render.ends_with(mce)
            },
        }
    }

    pub fn update_syntax(&mut self) {
        self.highlight = iter::repeat(Highlight::Normal)
            .take(self.render.chars().count()).collect();

        if self.syntax.is_none() { return }
        let syntax = self.syntax.as_ref().unwrap();

        let scs = syntax.singleline_comment_start;
        let mcs = syntax.multiline_comment_start;
        let mce = syntax.multiline_comment_end;

        let mut prev_sep = true;
        let mut in_string = None;
        let mut in_mlcomment = self.open_comment;
        let opens_comment = (!scs.is_empty() && self.render.contains(scs)) ||
            (!mcs.is_empty() && self.render.contains(mcs));

        let mut iter = self.render.chars().enumerate();

        let render = self.render.clone();
        let keyword_matches = syntax.keywords.iter().flat_map(|&kw| {
            render.match_indices(kw.as_str())
                .map(|pair| pair.0)
                .zip(iter::repeat(kw))
        }).collect::<Vec<(usize, Keyword)>>();

        while let Some((i, c)) = iter.next() {
            let prev_hl = if i > 0 {
                self.highlight[i - 1]
            } else {
                Highlight::Normal
            };

            if in_string.is_none() && !in_mlcomment && opens_comment {
                if self.render.chars().skip(i).collect::<String>().starts_with(scs) {
                    for j in i..self.highlight.len() {
                        self.highlight[j] = Highlight::Comment;
                    }
                    break;
                }
            }

            if in_string.is_none() && !mcs.is_empty() && !mce.is_empty() {
                if in_mlcomment {
                    match self.render.chars().skip(i).collect::<String>().find(mce) {
                        Some(j) => {
                            self.highlight[i] = Highlight::MLComment;
                            for k in 1..j + mce.len() {
                                self.highlight[i + k] = Highlight::MLComment;
                                iter.next();
                            }
                            in_mlcomment = false;
                            continue;
                        }
                        None => {
                            for j in i..self.highlight.len() {
                                self.highlight[j] = Highlight::MLComment;
                            }
                            break;
                        },
                    }
                } else if opens_comment {
                    if self.render.chars().skip(i).collect::<String>().starts_with(mcs) {
                        self.highlight[i] = Highlight::MLComment;
                        for j in 1..mcs.len() {
                            self.highlight[i + j] = Highlight::MLComment;
                            iter.next();
                        }
                        in_mlcomment = true;
                        continue;
                    }
                }
            }

            if syntax.flags.contains(&Flag::HighlightStrings) {
                match in_string.as_ref().map(|is: &InString| is.to_char()) {
                    None => {
                        let is = InString::from_char(c);
                        if is.is_some() {
                            in_string = is;
                            self.highlight[i] = Highlight::String;
                            continue;
                        }
                    },
                    Some(quote) => {
                        self.highlight[i] = Highlight::String;

                        if c == '\\' {
                            match iter.next() {
                                Some((j, _)) => {
                                    self.highlight[j] = Highlight::String;
                                    continue;
                                },
                                None => (),
                            }
                        }

                        if c == quote { in_string = None }
                        prev_sep = true;
                        continue;
                    },
                }
            }

            if syntax.flags.contains(&Flag::HighlightNumbers) {
                if (c.is_digit(10) && (prev_sep || prev_hl == Highlight::Number)) || (c == '.' && prev_hl == Highlight::Number) {
                    prev_sep = false;
                    self.highlight[i] = Highlight::Number;
                    continue;
                }
            }

            if prev_sep {
                match keyword_matches.iter().find(|&&(j, _)| { i == j }) {
                    None => (),
                    Some(&(_, kw)) => {
                        let s = kw.as_str();
                        let chars = self.render.chars().collect::<Vec<char>>();
                        let separated = i + s.len() == self.render.len() ||
                            is_separator(chars[i + s.len()]);
                        if !separated { continue }
                        let hl = Highlight::from_keyword(&kw);
                        self.highlight[i] = hl;
                        for j in 1..s.len() {
                            self.highlight[i + j] = hl;
                            iter.next();
                        }
                        continue;
                    },
                };
            }

            prev_sep = is_separator(c);
        }
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        self.contents.insert(at, c);
        self.update();
    }

    pub fn delete_char(&mut self, at: usize) {
        if at >= self.contents.len() { return }
        self.contents.remove(at);
        self.update();
    }

    pub fn append_string(&mut self, s: &str) {
        self.contents.push_str(s);
        self.update();
    }

    pub fn split_off(&mut self, at: usize) -> String {
        let remainder = util::safe_split_off(&mut self.contents, at);
        self.update();
        remainder
    }

    pub fn rendered_cursor_x(&self, cursor_x: usize) -> usize {
        self.contents.chars()
            .take(cursor_x)
            .fold(0, |acc, c| {
                if c == '\t' {
                    acc + KILO_TAB_STOP - (acc % KILO_TAB_STOP)
                } else {
                    acc + 1
                }
        })
    }

    pub fn raw_cursor_x(&self, rendered_x: usize) -> usize {
        self.contents.chars()
            .scan(0, |acc, c| {
                if c == '\t' {
                    *acc = *acc + KILO_TAB_STOP - (*acc % KILO_TAB_STOP)
                } else {
                    *acc += 1
                };
                Some(*acc)
            }).position(|rx| rx > rendered_x).unwrap()
    }

    fn update_render(&mut self) {
        self.render = Self::render_string(self.contents.clone());
    }

    fn render_string(s: String) -> String {
        let mut idx = 0;
        let renderer = |c|
            if c == '\t' {
                let n = KILO_TAB_STOP - (idx % KILO_TAB_STOP);
                idx += n;
                iter::repeat(' ').take(n)
            } else {
                idx += 1;
                // This is the same as iter::once(c), but the types of
                // the branches of the conditional have to line up.
                iter::repeat(c).take(1)
            };
        s.chars().flat_map(renderer).collect()
    }
}
