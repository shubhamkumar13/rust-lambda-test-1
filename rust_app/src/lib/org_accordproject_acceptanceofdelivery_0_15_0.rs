use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::org_accordproject_contract_0_2_0::*;
use crate::org_accordproject_runtime_0_2_0::*;
use crate::org_accordproject_organization_0_2_0::*;
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct InspectDeliverable {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "deliverableReceivedAt",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub deliverable_received_at: DateTime<Utc>,
   
   #[serde(
      rename = "inspectionPassed",
   )]
   pub inspection_passed: bool,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InspectionStatus {
   #[allow(non_camel_case_types)]
   PASSED_TESTING,
   #[allow(non_camel_case_types)]
   FAILED_TESTING,
   #[allow(non_camel_case_types)]
   OUTSIDE_INSPECTION_PERIOD,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionResponse {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "status",
   )]
   pub status: InspectionStatus,
   
   #[serde(rename = "shipper")]
   pub shipper: Organization,
   
   #[serde(rename = "receiver")]
   pub receiver: Organization,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceOfDeliveryClause {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(rename = "shipper")]
   pub shipper: Organization,
   
   #[serde(rename = "receiver")]
   pub receiver: Organization,
   
   #[serde(
      rename = "deliverable",
   )]
   pub deliverable: String,
   
   #[serde(
      rename = "businessDays",
   )]
   pub business_days: u64,
   
   #[serde(
      rename = "attachment",
   )]
   pub attachment: String,
   
   #[serde(
      rename = "clauseId",
   )]
   pub clause_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

