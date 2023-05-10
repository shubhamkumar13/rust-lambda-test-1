use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct PostalAddress {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "streetAddress",
      skip_serializing_if = "Option::is_none",
   )]
   pub street_address: Option<String>,
   
   #[serde(
      rename = "postalCode",
      skip_serializing_if = "Option::is_none",
   )]
   pub postal_code: Option<String>,
   
   #[serde(
      rename = "postOfficeBoxNumber",
      skip_serializing_if = "Option::is_none",
   )]
   pub post_office_box_number: Option<String>,
   
   #[serde(
      rename = "addressRegion",
      skip_serializing_if = "Option::is_none",
   )]
   pub address_region: Option<String>,
   
   #[serde(
      rename = "addressLocality",
      skip_serializing_if = "Option::is_none",
   )]
   pub address_locality: Option<String>,
   
   #[serde(
      rename = "addressCountry",
      skip_serializing_if = "Option::is_none",
   )]
   pub address_country: Option<String>,
}

