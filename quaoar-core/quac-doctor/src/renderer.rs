use std::{io::{self, BufWriter, Write}, path::PathBuf};

use ecow::EcoVec;

pub(crate) struct LineSpan{
    /// The start of the line based on the source
    pub line_start: usize,
    /// The end of the line based on the source
    pub line_end: usize,
    /// The line's byte span representation
    pub line_bytes: EcoVec<u8>,
    /// The number of the line
    pub line_number: usize
}

pub(crate) struct Problematic{
    pub start: usize,
    pub end: usize,
    pub line: LineSpan
}

/// Struct used to render error messages on a standardized format.
pub(crate) struct ViewRenderer{
    /// The error message
    pub msg: String,
    /// The path to the problematic file
    pub path: PathBuf,
    /// The span where the problematic is located
    pub problematic: Problematic,
}

impl ViewRenderer{
    pub fn new(msg: String, path: PathBuf, line_span: LineSpan, start: usize, end: usize) -> Self{
        Self { 
            msg, 
            path, 
            problematic: Problematic {
                start, 
                end, 
                line: line_span
            } 
        }
    }

    /// Renders the line under the problematic part of the line
    fn render_problematic_squeazy(&self) -> String{
        let mut b: EcoVec<u8> = EcoVec::new();
        let line = &self.problematic.line;

        for i in line.line_start..line.line_end{
            if i >= self.problematic.start && i < self.problematic.end { 
                b.push(b'^');
                continue;
            }
            b.push(b' ');
        }

        return String::from_utf8(b.to_vec()).unwrap();
    }

    /// Renders the error message in a beautiful way.
    /// 
    /// # Example:
    /// ```plaintext
    /// Error on src/main.qo:
    ///     2.  int stmt = 1+"2"
    ///                      ^^^ (QO#1) can't use strings on binary expressions
    /// 
    /// Maybe remove the quotes on the second value of the expression?
    /// ```
    pub fn render(self) -> io::Result<()>{
        let handle = io::stdout().lock();
        let mut writer = BufWriter::new(handle);

        let path = self.path.as_os_str();
        let line = &self.problematic.line;
        let line_string = String::from_utf8(line.line_bytes.to_vec()).unwrap();
        let suggestion = "PLACEHOLDER UNTIL IMPLEMENTED";

        write!(writer, "
Error on {:?}, line {}:

    {}
    {} {}

{}
        ", path, line.line_number, line_string, self.render_problematic_squeazy(), self.msg, suggestion)?;

        writer.flush()?;
        Ok(())
    }
}