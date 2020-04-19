mod chart;
pub use chart::{ load_daily, load_daily_range, Result };

mod realtime;
pub use realtime::{ PricingData };

mod web_scraper;
pub use web_scraper::scrape;