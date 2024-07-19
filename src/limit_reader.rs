use std::io;
use std::io::Read;

pub struct LimitReader<T: Read> {
    reader: T,
    limit: usize,
    bytes_seen: usize,
}

impl<T: Read> LimitReader<T> {
    pub fn new(reader: T, limit: usize) -> Self {
        LimitReader {
            reader,
            limit,
            bytes_seen: 0,
        }
    }
}

impl<T: Read> Read for LimitReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        assert!(self.bytes_seen <= self.limit);
        if self.bytes_seen == self.limit {
            // done forever
            return Ok(0);
        }
        let max_read = self.limit.min(buf.len()); // min of limit and buf.len()
        let bytes_read = self.reader.read(&mut buf[..max_read])?;
        self.bytes_seen += bytes_read;
        Ok(bytes_read)
    }
}

