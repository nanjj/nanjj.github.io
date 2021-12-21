use anyhow::Result;
use chrono::{prelude::DateTime, Local};
use clap::{App, Arg, SubCommand};
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
    process,
};

fn modified<P: AsRef<Path>>(p1: P, p2: P) -> Result<bool> {
    Ok(FileTime::from_last_modification_time(&metadata(p1)?)
        > FileTime::from_last_modification_time(&metadata(p2)?))
}

fn last_modification_time<P: AsRef<Path>>(p: P) -> Result<String> {
    let dt = DateTime::<Local>::from(
        UNIX_EPOCH
            + Duration::from_secs(
                FileTime::from_last_modification_time(&metadata(p)?).seconds() as u64,
            ),
    );
    Ok(format!(
        "{}",
        dt.format("<div style=\"text-align: right\"><code>%x %A %_I%p</code></div>")
    ))
}

fn insert_timestamp(content: &str, timestamp: &str) -> Result<String> {
    let mut s = String::new();
    let mut lines = content.lines();
    let mut next_line = || loop {
        if let Some(line) = lines.next() {
            if line != "" {
                break line;
            }
            continue;
        } else {
            break "";
        }
    };
    let line1 = next_line();
    if !line1.contains("`") && !line1.contains("<code>") {
        s.push_str(&format!("{}\n\n", timestamp));
    }
    s.push_str(&format!("{}\n", line1));
    while let Some(line) = lines.next() {
        s.push_str(line);
        s.push('\n');
    }
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
            let created_at = last_modification_time(src.join(path))?;
            let timestamp = format!("{}", created_at);
            ch.content = insert_timestamp(&ch.content, &timestamp)?;
            if ch.parent_names.len() == 0 {
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
    let matches = App::new("mdbook-blog")
        .about("A mdbook preprocessor for blog")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("check whether a renderer is supported"),
        )
        .get_matches();
    if let Some(supports) = matches.subcommand_matches("supports") {
        let renderer = supports.value_of("renderer").expect("Required argument");
        if renderer != "" {
            process::exit(0);
        }
        process::exit(1);
    }
    run().unwrap_or_default();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::io::Read;
    #[test]
    fn test_newer_than() {
        assert!(modified("not-exists-1", "not-exists-2").is_err());
        let res = modified("src/main.rs", "src/main.rs");
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
            "<head><meta http-equiv=\"refresh\" content=\"\
			 0;url=2021/euler.html\"></head>",
            s
        );
        remove_file(index).unwrap();
    }
    #[test]
    fn test_insert_timestamp() {
        let text = "hello\nworld!";
        let res = insert_timestamp(text, "timestamp").unwrap();
        assert_eq!("timestamp\n\nhello\nworld!\n", res);
    }

    #[test]
    fn test_creation_time() {
        let ctime = last_modification_time("src/main.rs");
        assert_ne!(String::from(""), ctime.unwrap());
    }
}
