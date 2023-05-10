use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "contractId",
   )]
   pub contract_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clause {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "clauseId",
   )]
   pub clause_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

