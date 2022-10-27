use jparse::*;
#[test]
fn jparse() -> Result<(), Box<dyn std::error::Error>> {
    let path = "JB1_3567.jpg";
    let warmup = 100;
    let runs = 10000;

    let mut total = std::time::Duration::default();
    let mut count = 0;
    while count < runs {
        let file = std::fs::File::open(&path)?;
        let len = file.metadata()?.len();
        let buffered_file = std::io::BufReader::new(file);
        let time = std::time::Instant::now();
        jpeg_size_buffered(len as usize, buffered_file)?;
        if count > warmup {
            total += time.elapsed();
        }
        count += 1;
    }
    println!(
        "buffered took {}ms for {runs} runs and had {warmup} warmup runs",
        total.as_millis()
    );

    let mut total = std::time::Duration::default();
    let mut count = 0;
    while count < runs {
        let time = std::time::Instant::now();
        jpeg_size(&path)?;
        if count > warmup {
            total += time.elapsed();
        }
        count += 1;
    }
    println!(
        "buffered took {}ms for {runs} runs and had {warmup} warmup runs",
        total.as_millis()
    );

    Ok(())
}
