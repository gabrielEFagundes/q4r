use crate::LINE_FEED;

pub struct SourceMap{
    pub lines: Vec<u8>
}

pub fn feed_lines(source: Vec<u8>) -> SourceMap{
    let mut smap = SourceMap{ lines: Vec::from([0]) }; // first byte, first line

    for s in 0..source.len(){
        if source[s] == LINE_FEED { smap.lines.push(source[s+1]); }
    }

    smap
}