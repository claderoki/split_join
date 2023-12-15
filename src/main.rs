use std::fs::File;
use std::{fs, io};
use std::io::{BufReader, Read, Write};
use std::path::Path;

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

fn join(path: &Path) -> io::Result<()> {
    // let file = File::create("").unwrap();

    let paths = fs::read_dir(path)
        .unwrap()
        .flatten()
        .map(|p|p.path())
        
        ;

    for path in paths {
        if path.as_path().extension().filter(|e|e == &"split").is_some() {
            println!("Name: {}", path.display())
        }
    }

    // let mut i = 0;
    // let mut reader = BufReader::new(File::open(path)?);
    // split_stream(path.metadata()?.len(), limit, |s| {
    //     let mut f = File::create(format_part(i + 1, path).unwrap())?;
    //     i += 1;
    //     split_stream(s, FileSize::of_mega_bytes(5), |t| {
    //         let mut buffer = vec![0; t as usize];
    //         reader.read_exact(&mut buffer)?;
    //         f.write_all(&buffer)?;
    //         Ok(())
    //     })
    // })
    Ok(())
}

fn main() {



    // let path = Path::new("C:/Users/Clark/Desktop/files/hjsplit.exe");
    // split(path, FileSize::of_kilo_bytes(40)).unwrap();

    let base = Path::new("C:/Users/Clark/Desktop/files");
    join(base);
}