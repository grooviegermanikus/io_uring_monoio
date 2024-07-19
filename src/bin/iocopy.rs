use std::env::args;
use std::fs::File;
use std::future::Future;
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fs, io};
use std::str::FromStr;

struct Source {

}


fn main() -> anyhow::Result<()> {

    let filename = PathBuf::from_str("copyout.dat").unwrap();

    //  dd if=/dev/urandom of=bigfile.dat bs=1M count=512
    let mut input = BufReader::new(File::open("bigfile.dat")?);

    let mut out = BufWriter::new(File::create(&filename)?);
    let started_at = Instant::now();
    io::copy(&mut input, &mut out)?;
    out.flush()?;
    let elapsed = started_at.elapsed();
    println!("wrote {} bytes in {:?} - {:.2}Mbps", fs::metadata(&filename)?.len(), elapsed, fs::metadata(&filename)?.len() as f64 / elapsed.as_secs_f64() / 1024.0 / 1024.0);

    Ok(())
}
