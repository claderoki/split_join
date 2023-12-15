use std::fs::{File, ReadDir};
use std::{fs, io};
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

struct FileSize {}

impl FileSize {
    fn of_kilo_bytes(amount: u64) -> u64 {
        1024 * amount
    }
    fn of_mega_bytes(amount: u64) -> u64 {
        FileSize::of_kilo_bytes(1024 * amount)
    }
    fn of_giga_bytes(amount: u64) -> u64 {
        FileSize::of_mega_bytes(1024 * amount)
    }
}

fn split_stream<F>(total: u64, limit: u64, mut consumer: F) -> io::Result<()>
where
    F: FnMut(u64) -> io::Result<()>,
{
    for _ in 0..total / limit {
        consumer(limit)?;
    }
    let remainder = total % limit;
    if remainder > 0 {
        consumer(remainder)?;
    }
    Ok(())
}

fn format_part(number: usize, path: &Path) -> Option<String> {
    Some(format!("{}.part_{}.split", path.to_str()?, number))
}

fn write_into(size: u64, file: &mut File, reader: &mut BufReader<File>) -> io::Result<()> {
    split_stream(size, FileSize::of_mega_bytes(5), |t| {
        let mut buffer = vec![0; t as usize];
        reader.read_exact(&mut buffer)?;
        file.write_all(&buffer)?;
        Ok(())
    })
}

fn split(path: &Path, limit: u64) -> io::Result<()> {
    let mut i = 0;
    let mut reader = BufReader::new(File::open(path)?);
    split_stream(path.metadata()?.len(), limit, |s| {
        let mut file = File::create(format_part(i + 1, path).unwrap())?;
        i += 1;
        write_into(s, &mut file, &mut reader)
    })
}

fn shift_extension(path: &str) {
    // path.
}

fn join(path: &Path) -> io::Result<()> {
    let paths: Vec<PathBuf> = fs::read_dir(path)
        .unwrap()
        .flatten()
        .map(|p|p.path())
        .filter(|p|p.as_path().extension().filter(|e|e == &"split").is_some())
        .collect();

    let first = paths.get(0).expect("Not found.");
    let file = File::create(first.as_path()).unwrap();

    for path in paths {
        println!("Name: {}", path.display())
    }

    Ok(())
}

fn main() {



    // let path = Path::new("C:/Users/Clark/Desktop/files/hjsplit.exe");
    // split(path, FileSize::of_kilo_bytes(40)).unwrap();

    let base = Path::new("C:/Users/Clark/Desktop/files");
    join(base);
}