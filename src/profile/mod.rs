mod scraper;

pub struct Profile {
}
impl Profile {
   /// Create a new realtime price quote streamer and make the initial connection to Yahoo for data
   pub fn new(symbol: &str) -> Profile {
      scraper::load(symbol);

      Profile {}
   }
}
