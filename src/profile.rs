use crate::{ Error, yahoo };

pub struct Address {
   pub street: String,
   pub city: String,
   pub state: String,
   pub country: String,
   pub zip: String
}

pub struct Profile {
   pub name: String,
   pub summary: String,

   pub address: Option<Address>,

   pub industry: String,
   pub sector: String,

   pub website: String
}
impl Profile {
   /// Create a new realtime price quote streamer and make the initial connection to Yahoo for data
   pub fn new(symbol: &str) -> Result<Profile, Error> {
      let data = yahoo::scrape(symbol)?.quote_summary_store;

      Ok(Profile {
         name: data.quote_type.name,
         summary: data.summary_profile.summary,
         address: None,
         industry: data.summary_profile.industry,
         sector: data.summary_profile.sector,
         website: data.summary_profile.website
      })
   }
}
