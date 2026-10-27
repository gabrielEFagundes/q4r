use std::path::PathBuf;

use ecow::EcoVec;
use quac_core::{
    errors::error::QError, 
    linefeeder::SourceMap, 
    tokens::Span
};

use crate::{die, renderer::{LineSpan, ViewRenderer}};

pub fn diagnose(errors: Vec<QError>, source: Vec<u8>, sourcemap: SourceMap, path: PathBuf){
    for e in errors{
        let problematic_start = e.span.start;
        let problematic_end = e.span.end;

        let (line, line_start, line_end) = extract_line(&sourcemap, e.span, &source);
        let line_span = LineSpan{ line_bytes: line, line_start, line_end, line_number: e.span.line };

        match ViewRenderer::new(e.msg, path.clone(), line_span, problematic_start, problematic_end)
            .render(){
                Ok(_) => return,
                Err(_) => die!("rust: error when trying to render Quaoar's error message."),
            }
    }
}

/// Gets the line of the error.
/// 
/// The first line is always the first position on the sourcemap lines array.
/// This goes on until the last line:
/// 
/// ```plaintext
///     [0] - line 1
///     [1] - line 2
///     ...
/// ```
/// 
/// Returns an array of bytes representing the whole problematic line
/// and the starting/ending point of the line inside the source.
pub fn extract_line(sourcemap: &SourceMap, span: Span, source: &Vec<u8>) -> (EcoVec<u8>, usize, usize){
    let this_line_pos = sourcemap.lines[span.line-1]; // line always comes as 1 or more, so this never fails

    let next_start_pos = if sourcemap.lines.len() > span.line{
        sourcemap.lines[span.line]
        
    } else { source.len() };

    (EcoVec::from(source[this_line_pos..next_start_pos].trim_ascii_end()), this_line_pos, next_start_pos)
}