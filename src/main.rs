use filetime::FileTime;
use mdbook::{
    book::{BookItem, BookItem::Chapter},
    errors::Error,
    preprocess::{CmdPreprocessor, PreprocessorContext},
};
use serde_json::to_writer;
use std::{
    fs::{metadata, File},
    io::{stdin, stdout, BufWriter, Write},
    path::Path,
};

fn modified<P: AsRef<Path>>(p1: P, p2: P) -> Result<bool, Error> {
    Ok(FileTime::from_last_modification_time(&metadata(p1)?)
        > FileTime::from_last_modification_time(&metadata(p2)?))
}

fn write_index_html<P: AsRef<Path>>(index: P, target: &str) -> Result<(), Error> {
    let file = File::create(index)?;
    let mut writer = BufWriter::new(&file);
    write!(
        &mut writer,
        "<head><meta http-equiv=\"refresh\" content=\"0;url={}\"></head>",
        target
    )?;
    Ok(())
}

fn handle(ctx: &PreprocessorContext, section: &mut BookItem) -> Result<(), Error> {
    if let Chapter(ref ch) = *section {
        if ch.name == "Home" {
            if let Some(ref path) = ch.path {
                let ref src = ctx.config.book.src;
                let res = modified(src.join(path), src.join("index.html"));
                if let Ok(false) = res {
                    return Ok(());
                }
                write_index_html(
                    src.join("index.html"),
                    path.with_extension("html").to_str().unwrap(),
                )?;
            }
        }
    }
    Ok(())
}

fn run() -> Result<(), Error> {
    let (ctx, mut book) = CmdPreprocessor::parse_input(stdin())?;
    book.for_each_mut(|section: &mut BookItem| handle(&ctx, section).unwrap());
    to_writer(stdout(), &book)?;
    Ok(())
}

fn main() {
    run().unwrap_or_default();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::io::Read;
    #[test]
    fn test_newer_than() {
        assert!(modified(Path::new("not-exists-1"), Path::new("not-exists-2")).is_err());
        let res = modified(Path::new("index.html"), Path::new("index.html"));
        assert!(res.is_ok());
        assert_eq!(false, res.unwrap());
    }

    #[test]
    fn test_write_index_html() {
        let index = Path::new("/tmp/index.html");
        let target = "2021/euler.html";
        let res = write_index_html(index, target);
        assert!(res.is_ok());
        let mut file = File::open(index).unwrap();
        let mut s = String::new();
        let size = Read::read_to_string(&mut file, &mut s).unwrap();
        assert_eq!(72, size);
        assert_eq!(
            "<head><meta http-equiv=\"refresh\" content=\"0;url=2021/euler.html\"></head>",
            s
        );
        remove_file(index).unwrap();
    }
}
