use std::sync::Arc;
use std::time::Instant;
use monoio::buf::IoVecBuf;
use monoio::fs::File;

#[monoio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a file
    let file = File::create("bff-monoio.dat").await?;

    const BUFFER_SIZE: usize = 128 * 1024 * 1024;
    let mut buf = Vec::with_capacity(BUFFER_SIZE);
    buf.resize(BUFFER_SIZE, 0u8);
    fill_with_xor_prng(buf.as_mut());

    let shared = Arc::new(buf);

    let started_at = Instant::now();
    // Write some data
    let (res, buf) = file.write_at(shared, 0).await;
    let n = res?;
    assert_eq!(n, BUFFER_SIZE);


    // Sync data to the file system.
    file.sync_all().await?;

    let elapsed = started_at.elapsed();

    println!("wrote {} bytes in {:?} - {:02}Mbps", n, elapsed, n as f64 / elapsed.as_secs_f64() / 1024.0 / 1024.0);
    // Close the file
    file.close().await?;

    Ok(())
}

fn fill_with_xor_prng(binary: &mut [u8]) {
    let seed_n = binary.len();
    let mut state: u32 = 0xdeadbeef;
    for i_word in 0..seed_n / 4 {
        let mut x = state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        state = x;

        binary[i_word * 4 + 0] = (x >> 0) as u8;
        binary[i_word * 4 + 1] = (x >> 8) as u8;
        binary[i_word * 4 + 2] = (x >> 16) as u8;
        binary[i_word * 4 + 3] = (x >> 24) as u8;
    }
}
