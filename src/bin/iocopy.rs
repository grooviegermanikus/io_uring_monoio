use std::env::args;
use std::fs::File;
use std::future::Future;
use std::io::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fs, io};
use std::str::FromStr;
use io_uring_monoio::limit_reader::LimitReader;

struct Source {

}

impl Read for Source {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        buf.iter_mut().for_each(|b| *b = 42);
        Ok(buf.len())
    }
}


fn main() -> anyhow::Result<()> {

    let endless_source = Source{};
    let mut input = LimitReader::new(endless_source, 512 * 1024 * 1024);

    let filename = PathBuf::from_str("copyout.dat").unwrap();


    let mut out = BufWriter::new(File::create(&filename)?);
    let started_at = Instant::now();
    io::copy(&mut input, &mut out)?;
    out.flush()?;
    let elapsed = started_at.elapsed();
    println!("wrote {} bytes in {:?} - {:.2}Mbps", fs::metadata(&filename)?.len(), elapsed, fs::metadata(&filename)?.len() as f64 / elapsed.as_secs_f64() / 1024.0 / 1024.0);
    // (on mango-hetzner-01)
    // wrote 536870912 bytes in 306.926422ms - 1668.15Mbps

    Ok(())
}
