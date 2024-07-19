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
    let mut n_bytes = 1 * 1024 * 1024;
    while n_bytes < 500 * 1024 * 1024 * 1024 {
        println!("Allocating {} bytes ...", n_bytes);
        let alloc = vec![0u8; n_bytes];
        println!("succeeded with {} bytes ...", n_bytes);

        n_bytes = n_bytes * 10;
    }

    Ok(())
}
