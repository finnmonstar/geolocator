pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
pub struct Information {
    pub status: String,
    pub message: Option<String>,
    pub continent: Option<String>,
    #[serde(rename = "continentCode")]
    pub continent_code: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "regionName")]
    pub region_name: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    #[serde(rename = "zip")]
    pub zip_code: Option<String>,
    #[serde(rename = "lat")]
    pub latitude: Option<f64>,
    #[serde(rename = "lon")]
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub offset: Option<isize>,
    pub currency: Option<String>,
    pub isp: Option<String>,
    #[serde(rename = "org")]
    pub organization: Option<String>,
    pub r#as: Option<String>,
    pub asname: Option<String>,
    pub reverse: Option<String>,
    pub mobile: Option<bool>,
    pub proxy: Option<bool>,
    pub hosting: Option<bool>,
    pub query: Option<String>,
}
