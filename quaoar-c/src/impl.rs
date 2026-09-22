pub trait AppendTo{
    fn append(self, buf: &mut Vec<u8>);
}

impl AppendTo for u8{
    #[inline(always)]
    fn append(self, buf: &mut Vec<u8>) {
        buf.push(self);
    }
}

impl AppendTo for &[u8]{
    #[inline(always)]
    fn append(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }
}

impl AppendTo for &mut [u8]{
    #[inline(always)]
    fn append(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }
}