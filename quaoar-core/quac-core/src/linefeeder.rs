use crate::LINE_FEED;

/// Maps the starting position of each byte inside the source
/// 
/// Used mainly for error and warning rendering
#[derive(Debug)]
pub struct SourceMap{
    pub lines: Vec<usize>
}

/// Call this before the lexer to map every line
pub fn feed_lines(source: &Vec<u8>) -> SourceMap{
    let mut smap = SourceMap{ lines: Vec::from([0]) }; // first byte, first line

    for s in 0..source.len(){
        if source[s] == LINE_FEED { smap.lines.push(s+1); }
    }
    
    smap
}