use anyhow::Result;
use chrono::{prelude::DateTime, Local};
use filetime::FileTime;
use std::time::{Duration, UNIX_EPOCH};

use mdbook::{
    book::{BookItem, BookItem::Chapter},
    preprocess::{CmdPreprocessor, PreprocessorContext},
};
use serde_json::to_writer;
use std::{
    fs::{metadata, File},
    io::{stdin, stdout, BufWriter, Write},
    path::Path,
};

fn modified<P: AsRef<Path>>(p1: P, p2: P) -> Result<bool> {
    Ok(FileTime::from_last_modification_time(&metadata(p1)?)
        > FileTime::from_last_modification_time(&metadata(p2)?))
}

fn creation_time<P: AsRef<Path>>(p: P) -> Result<String> {
    if let Some(mtime) = FileTime::from_creation_time(&metadata(p)?) {
        let seconds: u64 = mtime.seconds() as u64;
        let d = UNIX_EPOCH + Duration::from_secs(seconds);
        let datetime = DateTime::<Local>::from(d);
        Ok(format!("{}", datetime.format("%Y-%m-%d %H:%M:%S")))
    } else {
        Ok(String::from(""))
    }
}

fn insert_timestamp(content: &str, timestamp: &str) -> Result<String> {
    let mut s = String::new();
    let mut lines = content.lines();
    let line1 = lines.next().unwrap_or("");
    s.push_str(line1);
    s.push('\n');
    let line2 = lines.next().unwrap_or("");
    if line2 != "" && line1 != "" {
        if !line2.contains("`") {
            s.push_str(&format!("`{}`", timestamp));
            s.push('\n');
        }
    }
    s.push_str(line1);
    s.push('\n');
    Ok(s)
}

fn write_index_html<P: AsRef<Path>>(index: P, target: &str) -> Result<()> {
    let file = File::create(index)?;
    let mut writer = BufWriter::new(&file);
    write!(
        &mut writer,
        "<head><meta http-equiv=\"refresh\" content=\"0;url={}\"></head>",
        target
    )?;
    Ok(())
}

fn handle(ctx: &PreprocessorContext, section: &mut BookItem) -> Result<()> {
    if let Chapter(ref mut ch) = *section {
        let ref src = ctx.config.book.src;
        if let Some(ref path) = ch.path {
            let created_at = creation_time(src.join(path))?;
            let timestamp = format!("{}", created_at);
            ch.content = insert_timestamp(&ch.content, &timestamp)?;
            if ch.name == "Home" {
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

fn run() -> Result<()> {
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
    #[test]
    fn test_insert_timestamp() {
        let text = "hello\nworld\n!";
        let res = insert_timestamp(text, "timestamp").unwrap();
        assert_eq!("hello\n`timestamp`\n!", res);
    }
}
