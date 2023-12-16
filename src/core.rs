use std::fs::File;
use std::{fs, io};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

struct Progress {
    total: u64,
    current: u64,
}

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

fn shift_by(by: char, path: &str, amount: usize) -> String {
    let mut i = path.len();
    let mut last_found = i;
    let mut found = 0;
    for char in path.chars().rev() {
        if found >= amount {
            break;
        }
        if char == by {
            last_found = i-1;
            found += 1;
        }
        i -= 1;
    }
    path[0..last_found].to_string()
}

fn join(path: &Path) -> io::Result<()> {
    let paths: Vec<PathBuf> = fs::read_dir(path)
        .unwrap()
        .flatten()
        .map(|p|p.path())
        .filter(|p|p.as_path().extension().filter(|e|e == &"split").is_some())
        .collect();

    let shifted = paths.get(0).and_then(|f|f.to_str()).map(|f|shift_by('.', f, 2)).unwrap();
    let joined_name = format!("{shifted}.join");
    let mut file = File::create(joined_name)?;

    for path in paths {
        let length = path.metadata()?.len();
        let mut reader = BufReader::new(File::open(path)?);
        write_into(length, &mut file, &mut reader)?;
    }

    Ok(())
}
