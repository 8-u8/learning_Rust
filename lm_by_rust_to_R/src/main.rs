use nalgebra::{DMatrix, Scalar};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

fn parse_csv<N, R>(input: R) -> Result<DMatrix<N>, Box<dyn std::error::Error>>
where
    N: FromStr + Scalar,
    N::Err: std::error::Error,
    R: BufRead,
{
    // initialize an empty vector to fill with numbers.
    let mut data = Vec::new();

    // initialize the number of rows to zero.
    let mut rows = 0;

    // for each line in the input,
    for line in input.lines() {
        rows += 1;

        for datum in line?.split_terminator(",") {
            data.push(N::from_str(datum.trim())?);
        }
    }
    let cols = data.len() / rows;
    Ok(DMatrix::from_row_slice(rows, cols, &data[..]))
}

fn listdir() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("starting dir: {}", path.display());

    let root = Path::new("/");
    assert!(env::set_current_dir(&root).is_ok());

    let path = env::current_dir()?;
    println!("final dir: {}", path.display());

    Ok(())
}

fn main() {
    listdir().unwrap();
    // todo filename from stdin.
    let file = File::open("hogehoge").unwrap();
    let bos: DMatrix<f64> = parse_csv(BufReader::new(file)).unwrap();
    println!("{}", bos.rows(0, 5));
}
