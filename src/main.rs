use mdbook::book::BookItem;
use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;
use serde_json;

use std::fs::File;
use std::io;
use std::io::prelude::*;
// use std::fmt::Write;
use std::path::Path;

fn process() -> Result<(), Error> {
    let (_ctx, mut book) = CmdPreprocessor::parse_input(io::stdin())?;
    book.for_each_mut(|section: &mut BookItem| {
        if let BookItem::Chapter(ref mut ch) = *section {
            if ch.name == "Home" {
                if let Some(ref mut path) = ch.path {
                    eprintln!("Get Home {:?}", path);
                    let idxpath = Path::new("src/index.html");
                    let idxdisplay = idxpath.display();
                    let mut idxfile = match File::create(&idxpath) {
                        Err(why) => panic!("couldn't create {}: {}", idxdisplay, why),
                        Ok(idxfile) => idxfile,
                    };
                    match idxfile.write_all(
                        "<head><meta http-equiv=\"refresh\" content=\"0;url=\"></head>".as_bytes(),
                    ) {
                        Err(why) => panic!("couldn't write to {}: {}", idxdisplay, why),
                        Ok(_) => (),
                    }
                }
            }
        }
    });
    serde_json::to_writer(io::stdout(), &book)?;
    Ok(())
}

fn main() {
    let _res = process();
}
