use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Flag {
    HighlightNumbers,
    HighlightStrings,
}

pub type Flags = HashSet<Flag>;

#[derive(Clone)]
pub struct Syntax {
    pub filetype: &'static str,
    pub filematch: Vec<&'static str>,
    pub keywords: Vec<Keyword>,
    pub singleline_comment_start: &'static str,
    pub multiline_comment_start: &'static str,
    pub multiline_comment_end: &'static str,
    pub flags: Flags,
}

#[derive(Clone, Copy)]
pub enum Keyword {
    One(&'static str),
    Two(&'static str),
}

impl Keyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            &Keyword::One(s) => s,
            &Keyword::Two(s) => s,
        }
    }
}

impl Syntax {
    fn database() -> Vec<Self> {
        let mut db = Vec::new();
        db.push(Syntax {
            filetype: "c",
            filematch: vec![".c", ".h", ".cpp"],
            keywords: vec![
                Keyword::One("switch"), Keyword::One("if"), Keyword::One("while"),
                Keyword::One("for"), Keyword::One("break"), Keyword::One("continue"),
                Keyword::One("return"), Keyword::One("else"), Keyword::One("struct"),
                Keyword::One("union"), Keyword::One("typedef"), Keyword::One("static"),
                Keyword::One("enum"), Keyword::One("class"), Keyword::One("case"),

                Keyword::Two("int"), Keyword::Two("long"), Keyword::Two("double"),
                Keyword::Two("float"), Keyword::Two("char"), Keyword::Two("unsigned"),
                Keyword::Two("signed"), Keyword::Two("void"),
            ],
            singleline_comment_start: "//",
            multiline_comment_start: "/*",
            multiline_comment_end: "*/",
            flags: [
                Flag::HighlightNumbers,
                Flag::HighlightStrings,
            ].iter().cloned().collect(),
        });
        db.push(Syntax {
            filetype: "rust",
            filematch: vec![".rs"],
            keywords: vec![
                Keyword::One("match"), Keyword::One("if"), Keyword::One("while"),
                Keyword::One("for"), Keyword::One("break"), Keyword::One("continue"),
                Keyword::One("return"), Keyword::One("else"), Keyword::One("struct"),
                Keyword::One("pub"), Keyword::One("const"), Keyword::One("static"),
                Keyword::One("enum"), Keyword::One("impl"), Keyword::One("use"),
                Keyword::One("fn"), Keyword::One("mod"), Keyword::One("let"),
                Keyword::One("mut"), Keyword::One("self"),

                Keyword::Two("usize"), Keyword::Two("isize"), Keyword::Two("str"),
                Keyword::Two("bool"), Keyword::Two("char"), Keyword::Two("String"),
                Keyword::Two("Option"), Keyword::Two("Vec"), Keyword::Two("Self"),
                Keyword::Two("u8"), Keyword::Two("u16"), Keyword::Two("u32"),
                Keyword::Two("i8"), Keyword::Two("i16"), Keyword::Two("i32"),
            ],
            singleline_comment_start: "//",
            multiline_comment_start: "/*",
            multiline_comment_end: "*/",
            flags: [
                Flag::HighlightNumbers,
                Flag::HighlightStrings,
            ].iter().cloned().collect(),
        });
        db
    }

    pub fn for_filename(filename: &str) -> Option<Self> {
        for s in Self::database().into_iter() {
            let res = s.filematch.iter()
                .map(|ext| filename.rfind(ext))
                .enumerate()
                .find(|&(_, opt)| opt.is_some());
            match res {
                Some((match_idx, Some(name_idx))) => {
                    let matched = s.filematch[match_idx];
                    if matched.chars().next().unwrap() == '.' ||
                        name_idx + matched.len() == filename.len() {
                        return Some(s)
                    }
                },
                Some((_, None)) => unreachable!(),
                None => continue,
            }
        }
        None
    }
}

