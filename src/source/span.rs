use std::cmp;
use std::fmt;
use std::rc::Rc;

use io;
use source::File;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub src: Rc<File>,
    pub start_byte: usize,
    pub end_byte: usize,
    pub line: u32,
    pub col: u32,
    pub width: u32,
    pub line_start_byte: usize,
    pub line_end_byte: usize,
}

fn find_eol_byte(file: &File, start: usize) -> usize {
    if start >= file.contents.len() {
        return start;
    }
    let offset = start;
    for (i, c) in file.contents[start..].char_indices() {
        let i = i + offset;
        if c == '\n' {
            return i;
        }
    }
    return file.contents.len();
}

impl Span {
    pub fn at_start_of(src: Rc<File>) -> Span {
        let line_end_byte = find_eol_byte(&*src, 0);
        Span {
            src,
            start_byte: 0,
            end_byte: 0,
            line: 0,
            col: 0,
            width: 0,
            line_start_byte: 0,
            line_end_byte,
        }
    }

    fn advance_start_to(&mut self, start_byte: usize) {
        assert!(start_byte >= self.start_byte);
        assert!(start_byte <= self.end_byte);
        let offset = self.start_byte;
        let mut chars = self.src.contents[self.start_byte..].char_indices();
        loop {
            let (i, c) = chars
                .next()
                .unwrap_or((self.src.contents.len() - offset, '\0'));
            let i = i + offset;
            if i == start_byte {
                break;
            }
            assert!(i < start_byte);
            // TODO: Feed in a file with tabs; see they are not spaced correctly; make tabs increment col by 6;
            //       still doesn't look right; write a complex algorithm to correctly space tabs for any width;
            //       realize unicode is a thing; look for a std lib function to give the width of a character;
            //       fail to find a satasfactory one; look for a crate; fail to find a satasfactory one;
            //       search for how everyone else does it; realize its a hard problem; realize its a Hard Problem;
            //       read through terminal source code; read through harfbuzz source code;
            //       study writing systems throught history; personally call every member of the unicode consortium
            //       go back to incrementing col by 1
            self.col += 1;
            self.width -= 1;
            if c == '\n' {
                self.line += 1;
                self.line_start_byte = chars
                    .clone()
                    .next()
                    .unwrap_or((self.src.contents.len(), '\0'))
                    .0
                    + offset;
                self.line_end_byte = find_eol_byte(&*self.src, self.line_start_byte);
            }
        }
        self.start_byte = start_byte;
    }

    fn advance_end_to(&mut self, end_byte: usize) {
        assert!(end_byte >= self.end_byte);
        assert!(end_byte <= self.src.contents.len());
        let offset = self.end_byte;
        let mut chars = self.src.contents[self.end_byte..].char_indices();
        loop {
            let (i, _) = chars
                .next()
                .unwrap_or((self.src.contents.len() - offset, '\0'));
            let i = i + offset;
            if i == end_byte {
                break;
            }
            self.width += 1;
        }
        self.end_byte = end_byte;
    }

    pub fn span_to_byte(&self, end: usize) -> Span {
        assert!(end > self.end_byte);
        assert!(end <= self.src.contents.len());
        let mut ret = self.clone();
        ret.advance_end_to(end);
        ret.advance_start_to(self.end_byte);
        return ret;
    }

    pub fn between(a: &Span, b: &Span) -> Span {
        assert_eq!(a.src, b.src);
        let mut ret = if a.start_byte < b.start_byte {
            a.clone()
        } else {
            b.clone()
        };
        ret.advance_end_to(cmp::max(a.end_byte, b.end_byte));
        return ret;
    }

    pub fn issue(&self, severity: io::Severity, message: &str) -> io::Issue {
        io::Issue {
            span: Some(self.clone()),
            severity,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}..{}",
            self.src.unwrap_path(),
            self.line,
            self.col,
            self.col + self.width
        )
    }
}

/*
impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start_byte, self.end_byte)
    }
}
*/

pub struct Generator {
    src: Rc<File>,
    current: usize,
}

impl Generator {
    pub fn new(src: Rc<File>) -> Generator {
        Generator { src, current: 0 }
    }

    pub fn skip(&mut self, bytes: i32) -> &mut Generator {
        self.current = (self.current as i32 + bytes) as usize;
        self
    }

    pub fn jump_to(&mut self, byte: usize) -> &mut Generator {
        self.current = byte;
        self
    }

    pub fn reset(&mut self) -> &mut Generator {
        self.jump_to(0)
    }

    pub fn span(&mut self, bytes: usize) -> Span {
        let start = self.current;
        self.skip(bytes as i32);
        let mut ret = Span::at_start_of(self.src.clone());
        ret.advance_end_to(start + bytes);
        ret.advance_start_to(start);
        return ret;
    }
}
