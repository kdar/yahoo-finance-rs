use serde::{ Deserialize };
use std::io::{ BufRead, BufReader };
use ureq;

use crate::{Error, Interval};

static BASE_URL: &str = "https://finance.yahoo.com/quote";
static DATA_VAR: &str = "root.App.main";

pub_json!(SummaryProfile {
   city: String,
   state: String,
   country: String,
   zip: String,
   sector: String,
   industry: String,
   website: String,
   #[serde(rename = "longBusinessSummary")] summary: String
});
pub_json!(QuoteSummaryStore { #[serde(rename = "summaryProfile")] summary_profile: SummaryProfile });
pub_json!(Stores { #[serde(rename = "QuoteSummaryStore")] quote_summary_store: QuoteSummaryStore });
pub_json!(Dispatcher { stores: Stores });
pub_json!(Context { dispatcher: Dispatcher });
pub_json!(Response { context: Context });

pub fn load(symbol: &str) -> std::result::Result<(), Error> {
   let url = format!("{url}/{symbol}", url=BASE_URL, symbol=symbol);
   let resp = ureq::get(url.as_str()).call();

   let reader = BufReader::new(resp.into_reader());
   let line = reader.lines()
                    .map(|line| line.unwrap())
                    .filter(|line| line.starts_with(DATA_VAR))
                    .next()
                    .expect("Need data");

   let data = line.trim_start_matches(DATA_VAR)
                  .trim_start_matches(|c| c == ' ' || c == '=')
                  .trim_end_matches(';');

   let response: Response = serde_json::from_str(data).unwrap();
   let profile = response.context.dispatcher.stores.quote_summary_store.summary_profile;
   println!("{} - {}", profile.sector, profile.industry);

   Ok(())
}
