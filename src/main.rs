use mdbook::{book::BookItem, errors::Error, preprocess::CmdPreprocessor};
use serde_json;

use std::{
    fs::File,
    io,
    io::{BufWriter, Write},
    path::PathBuf,
};

fn process() -> Result<(), Error> {
    let (_ctx, mut book) = CmdPreprocessor::parse_input(io::stdin())?;
    book.for_each_mut(|section: &mut BookItem| {
        if let BookItem::Chapter(ref ch) = *section {
            if ch.name == "Home" {
                if let Some(ref path) = ch.path {
                    let mut cpath = PathBuf::from(path);
                    cpath.set_extension("html");
                    if let Some(ref idxuri) = cpath.to_str() {
                        eprintln!("Get Home {}", idxuri);
                        let file = File::create("src/index.html").unwrap();
                        let mut writer = BufWriter::new(&file);
                        write!(
                            &mut writer,
                            "<head><meta http-equiv=\"refresh\" content=\"0;url={}\"></head>",
                            idxuri
                        )
                        .unwrap();
                    }
                }
            }
        }
    });
    serde_json::to_writer(io::stdout(), &book)?;
    Ok(())
}

fn main() {
    let _ = process();
}
