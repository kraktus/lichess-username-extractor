use indicatif::ProgressBar;
use pgn_reader::{RawHeader, Skip, Visitor};
use rustc_hash::FxHashSet;

pub struct Usernames {
    pub games: usize,
    pub usernames: FxHashSet<String>,
    pb: ProgressBar,
}

impl Usernames {
    pub fn new(pb: ProgressBar) -> Self {
        Self {
            games: 0,
            pb,
            usernames: FxHashSet::default()
        }
    }
}

impl Visitor for Usernames {
    type Result = ();

    fn begin_game(&mut self) {
        self.games += 1;
        if self.games % 10_000 == 9999 {
            self.pb.inc(10_000)
        }

    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"White" || key == b"Black" {
            self.usernames.insert(
                value
                    .decode_utf8()
                    .unwrap_or_else(|e| {
                        panic!(
                            "{}",
                            format!("Error {e} decoding username at game: {}", self.games)
                        )
                    })
                    .to_string(),
            );
        }
    }

    fn end_headers(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {}
}
