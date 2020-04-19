use serde::Deserialize;

use std::result;
use std::io::{ BufRead, BufReader };
use ureq;

use crate::Error;

static DATA_VAR: &str = "root.App.main";

ez_serde!(QuoteType { #[serde(rename = "longName")] name: String });

ez_serde!(SummaryProfile {
   #[serde(rename = "address1")]
   address: String,
   city: String,
   state: String,
   country: String,
   zip: String,

   #[serde(rename = "fullTimeEmployees")]
   employees: u32,

   sector: String,
   industry: String,

   #[serde(rename = "longBusinessSummary")]
   summary: String,
   website: String
});

ez_serde!(QuoteSummaryStore {
   #[serde(rename = "summaryProfile")] summary_profile: SummaryProfile,
   #[serde(rename = "quoteType")] quote_type: QuoteType
});
ez_serde!(Stores { #[serde(rename = "QuoteSummaryStore")] quote_summary_store: QuoteSummaryStore });
ez_serde!(Dispatcher { stores: Stores });
ez_serde!(Context { dispatcher: Dispatcher });
ez_serde!(Response { context: Context });

pub fn scrape<'a>(symbol: &'a str) -> result::Result<Stores, Error> {
   let url = format!("https://finance.yahoo.com/quote/{symbol}", symbol=symbol);
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

   let response: Response = serde_json::from_str(data).expect("Invalid structure");

   Ok(response.context.dispatcher.stores)
}
