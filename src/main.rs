//! A simple unique username extractor in rust from the lichess db

use std::{
    collections::HashSet,
    env,
    fs::File,
    io, mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use pgn_reader::{BufferedReader, RawHeader, San, SanPlus, Skip, Visitor};

mod visitor;

fn main() {
    for arg in env::args().skip(1) {
        let file = File::open(&arg).expect("fopen");

        let uncompressed: Box<dyn io::Read> = if arg.ends_with(".zst") {
            Box::new(zstd::Decoder::new(file).expect("zst decoder"))
        } else if arg.ends_with(".bz2") {
            Box::new(bzip2::read::MultiBzDecoder::new(file))
        } else if arg.ends_with(".xz") {
            Box::new(xz2::read::XzDecoder::new(file))
        } else if arg.ends_with(".gz") {
            Box::new(flate2::read::GzDecoder::new(file))
        } else if arg.ends_with(".lz4") {
            Box::new(lz4::Decoder::new(file).expect("lz4 decoder"))
        } else {
            Box::new(file)
        };
        let mut reader = BufferedReader::new(uncompressed);
        let mut visitor = visitor::Usernames::default();
        reader.read_all(&mut visitor).expect("Valid pgn file");
    }
}
