use filetime::FileTime;
use mdbook::{
    book::{BookItem, BookItem::Chapter},
    errors::Error,
    preprocess::CmdPreprocessor,
};
use serde_json::to_writer;
use std::{
    fs::{metadata, File},
    io,
    io::{BufWriter, Write},
    path::PathBuf,
};

fn handle(section: &mut BookItem) -> Result<(), Error> {
    if let Chapter(ref ch) = *section {
        if ch.name == "Home" {
            if let Some(ref path) = ch.path {
                let mut cpath = PathBuf::from(path);
                let mut srcmd = PathBuf::from("src");
                srcmd.push(cpath.to_str().unwrap());
                let md1 = metadata(srcmd)?;
                let md2 = metadata("src/index.html")?;
                if FileTime::from_last_modification_time(&md1)
                    <= FileTime::from_last_modification_time(&md2)
                {
                    return Ok(());
                }
                cpath.set_extension("html");
                if let Some(ref idxuri) = cpath.to_str() {
                    eprintln!("Get Home {}", idxuri);
                    let file = File::create("src/index.html")?;
                    let mut writer = BufWriter::new(&file);
                    write!(
                        &mut writer,
                        "<head><meta http-equiv=\"refresh\" content=\"0;url={}\"></head>",
                        idxuri
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn process() -> Result<(), Error> {
    let (_ctx, mut book) = CmdPreprocessor::parse_input(io::stdin())?;
    book.for_each_mut(|section: &mut BookItem| handle(section).unwrap());
    to_writer(io::stdout(), &book)?;
    Ok(())
}

fn main() {
    process().unwrap();
}
