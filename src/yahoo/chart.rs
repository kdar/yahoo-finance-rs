use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize};
use ureq;

use crate::{Error, Interval};

static BASE_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";

ez_serde!(Meta {
   symbol: String,

   #[serde(with = "ts_seconds")]
   first_trade_date: DateTime<Utc>,

   #[serde(rename = "regularMarketPrice")]
   current_price: f32,

   #[serde(rename = "chartPreviousClose")]
   previous_close: f32
});
ez_serde!(OHLCV { open: Vec<Option<f64>>, high: Vec<Option<f64>>, low: Vec<Option<f64>>, close: Vec<Option<f64>>, volume: Vec<Option<u64>> });
ez_serde!(Indicators { #[serde(rename = "quote")] quotes: Vec<OHLCV> });
ez_serde!(Result { meta: Meta, #[serde(rename = "timestamp")] timestamps: Vec<u64>, indicators: Indicators });

ez_serde!(Err { code: String, description: String });
ez_serde!(Chart { result: Option<Vec<Result>>, error: Option<Err> });
ez_serde!(Response { chart: Chart });

fn build_chart(x: &str) -> Chart {
   let response: Response = serde_json::from_str(x).unwrap();
   response.chart
}

fn load(url: &str) -> std::result::Result<Result, Error> {
   let resp = ureq::get(url).call();
   if resp.ok() {
      let result = build_chart(&resp.into_string().unwrap()).result.unwrap();
      Ok(result[0].clone())
   } else {
      let error = build_chart(&resp.into_string().unwrap()).error.unwrap();
      Err(Error::Other { code: error.code, description: error.description })
   }
}

pub fn load_daily(symbol: &str, interval: Interval) -> std::result::Result<Result, Error> {
   let url = format!("{url}/{symbol}?symbol={symbol}&range={period}&interval=1d", url=BASE_URL, symbol=symbol, period=interval);
   load(&url)
}

pub fn load_daily_range(symbol: &str, start: i64, end: i64) -> std::result::Result<Result, Error> {
   let url = format!("{url}/{symbol}?symbol={symbol}&period1={start}&period2={end}&interval=1d", url=BASE_URL, symbol=symbol, start=start, end=end);
   load(&url)
}