use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::org_accordproject_geo_0_2_0::*;
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "identifier",
   )]
   pub identifier: String,
   
   #[serde(
      rename = "name",
      skip_serializing_if = "Option::is_none",
   )]
   pub name: Option<String>,
   
   #[serde(
      rename = "description",
      skip_serializing_if = "Option::is_none",
   )]
   pub description: Option<String>,
   
   #[serde(
      rename = "duns",
      skip_serializing_if = "Option::is_none",
   )]
   pub duns: Option<String>,
   
   #[serde(
      rename = "place",
      skip_serializing_if = "Option::is_none",
   )]
   pub place: Option<Place>,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

