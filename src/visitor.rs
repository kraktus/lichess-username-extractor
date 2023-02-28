use pgn_reader::{BufferedReader, RawHeader, San, SanPlus, Skip, Visitor};
use rustc_hash::FxHashSet;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

#[derive(Default)]
pub struct Usernames {
    pub nb_game: usize,
    pub usernames: FxHashSet<String>,
}

impl Visitor for Usernames {
    type Result = ();

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"White" || key == b"Black" {
            self.usernames.insert(
                value
                    .decode_utf8()
                    .unwrap_or_else(|e| {
                        panic!("{}", format!(
                            "Error {e} decoding username at game: {}",
                            self.nb_game
                        ))
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
