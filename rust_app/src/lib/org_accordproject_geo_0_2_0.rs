use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::org_accordproject_address_0_2_0::*;
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct GeoCoordinates {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "address",
      skip_serializing_if = "Option::is_none",
   )]
   pub address: Option<PostalAddress>,
   
   #[serde(
      rename = "addressCountry",
      skip_serializing_if = "Option::is_none",
   )]
   pub address_country: Option<String>,
   
   #[serde(
      rename = "elevation",
      skip_serializing_if = "Option::is_none",
   )]
   pub elevation: Option<f64>,
   
   #[serde(
      rename = "latitude",
      skip_serializing_if = "Option::is_none",
   )]
   pub latitude: Option<f64>,
   
   #[serde(
      rename = "longitude",
      skip_serializing_if = "Option::is_none",
   )]
   pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "address",
      skip_serializing_if = "Option::is_none",
   )]
   pub address: Option<PostalAddress>,
   
   #[serde(
      rename = "branchCode",
      skip_serializing_if = "Option::is_none",
   )]
   pub branch_code: Option<String>,
   
   #[serde(
      rename = "faxNumber",
      skip_serializing_if = "Option::is_none",
   )]
   pub fax_number: Option<String>,
   
   #[serde(
      rename = "geo",
      skip_serializing_if = "Option::is_none",
   )]
   pub geo: Option<GeoCoordinates>,
   
   #[serde(
      rename = "globalLocationNumber",
      skip_serializing_if = "Option::is_none",
   )]
   pub global_location_number: Option<String>,
   
   #[serde(
      rename = "mapUrl",
      skip_serializing_if = "Option::is_none",
   )]
   pub map_url: Option<String>,
   
   #[serde(
      rename = "isicsV4",
      skip_serializing_if = "Option::is_none",
   )]
   pub isics_v4: Option<String>,
   
   #[serde(
      rename = "telephone",
      skip_serializing_if = "Option::is_none",
   )]
   pub telephone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "name",
   )]
   pub name: String,
   
   #[serde(
      rename = "optional",
   )]
   pub optional: CountryCodeISOAlpha2,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CountryCodeISOAlpha2 {
   #[allow(non_camel_case_types)]
   AD,
   #[allow(non_camel_case_types)]
   AE,
   #[allow(non_camel_case_types)]
   AF,
   #[allow(non_camel_case_types)]
   AG,
   #[allow(non_camel_case_types)]
   AI,
   #[allow(non_camel_case_types)]
   AL,
   #[allow(non_camel_case_types)]
   AM,
   #[allow(non_camel_case_types)]
   AO,
   #[allow(non_camel_case_types)]
   AQ,
   #[allow(non_camel_case_types)]
   AR,
   #[allow(non_camel_case_types)]
   AS,
   #[allow(non_camel_case_types)]
   AT,
   #[allow(non_camel_case_types)]
   AU,
   #[allow(non_camel_case_types)]
   AW,
   #[allow(non_camel_case_types)]
   AX,
   #[allow(non_camel_case_types)]
   AZ,
   #[allow(non_camel_case_types)]
   BA,
   #[allow(non_camel_case_types)]
   BB,
   #[allow(non_camel_case_types)]
   BD,
   #[allow(non_camel_case_types)]
   BE,
   #[allow(non_camel_case_types)]
   BF,
   #[allow(non_camel_case_types)]
   BG,
   #[allow(non_camel_case_types)]
   BH,
   #[allow(non_camel_case_types)]
   BI,
   #[allow(non_camel_case_types)]
   BJ,
   #[allow(non_camel_case_types)]
   BL,
   #[allow(non_camel_case_types)]
   BM,
   #[allow(non_camel_case_types)]
   BN,
   #[allow(non_camel_case_types)]
   BO,
   #[allow(non_camel_case_types)]
   BQ,
   #[allow(non_camel_case_types)]
   BR,
   #[allow(non_camel_case_types)]
   BS,
   #[allow(non_camel_case_types)]
   BT,
   #[allow(non_camel_case_types)]
   BV,
   #[allow(non_camel_case_types)]
   BW,
   #[allow(non_camel_case_types)]
   BY,
   #[allow(non_camel_case_types)]
   BZ,
   #[allow(non_camel_case_types)]
   CA,
   #[allow(non_camel_case_types)]
   CC,
   #[allow(non_camel_case_types)]
   CD,
   #[allow(non_camel_case_types)]
   CF,
   #[allow(non_camel_case_types)]
   CG,
   #[allow(non_camel_case_types)]
   CH,
   #[allow(non_camel_case_types)]
   CI,
   #[allow(non_camel_case_types)]
   CK,
   #[allow(non_camel_case_types)]
   CL,
   #[allow(non_camel_case_types)]
   CM,
   #[allow(non_camel_case_types)]
   CN,
   #[allow(non_camel_case_types)]
   CO,
   #[allow(non_camel_case_types)]
   CR,
   #[allow(non_camel_case_types)]
   CU,
   #[allow(non_camel_case_types)]
   CV,
   #[allow(non_camel_case_types)]
   CW,
   #[allow(non_camel_case_types)]
   CX,
   #[allow(non_camel_case_types)]
   CY,
   #[allow(non_camel_case_types)]
   CZ,
   #[allow(non_camel_case_types)]
   DE,
   #[allow(non_camel_case_types)]
   DJ,
   #[allow(non_camel_case_types)]
   DK,
   #[allow(non_camel_case_types)]
   DM,
   #[allow(non_camel_case_types)]
   DO,
   #[allow(non_camel_case_types)]
   DZ,
   #[allow(non_camel_case_types)]
   EC,
   #[allow(non_camel_case_types)]
   EE,
   #[allow(non_camel_case_types)]
   EG,
   #[allow(non_camel_case_types)]
   EH,
   #[allow(non_camel_case_types)]
   ER,
   #[allow(non_camel_case_types)]
   ES,
   #[allow(non_camel_case_types)]
   ET,
   #[allow(non_camel_case_types)]
   FI,
   #[allow(non_camel_case_types)]
   FJ,
   #[allow(non_camel_case_types)]
   FK,
   #[allow(non_camel_case_types)]
   FM,
   #[allow(non_camel_case_types)]
   FO,
   #[allow(non_camel_case_types)]
   FR,
   #[allow(non_camel_case_types)]
   GA,
   #[allow(non_camel_case_types)]
   GB,
   #[allow(non_camel_case_types)]
   GD,
   #[allow(non_camel_case_types)]
   GE,
   #[allow(non_camel_case_types)]
   GF,
   #[allow(non_camel_case_types)]
   GG,
   #[allow(non_camel_case_types)]
   GH,
   #[allow(non_camel_case_types)]
   GI,
   #[allow(non_camel_case_types)]
   GL,
   #[allow(non_camel_case_types)]
   GM,
   #[allow(non_camel_case_types)]
   GN,
   #[allow(non_camel_case_types)]
   GP,
   #[allow(non_camel_case_types)]
   GQ,
   #[allow(non_camel_case_types)]
   GR,
   #[allow(non_camel_case_types)]
   GS,
   #[allow(non_camel_case_types)]
   GT,
   #[allow(non_camel_case_types)]
   GU,
   #[allow(non_camel_case_types)]
   GW,
   #[allow(non_camel_case_types)]
   GY,
   #[allow(non_camel_case_types)]
   HK,
   #[allow(non_camel_case_types)]
   HM,
   #[allow(non_camel_case_types)]
   HN,
   #[allow(non_camel_case_types)]
   HR,
   #[allow(non_camel_case_types)]
   HT,
   #[allow(non_camel_case_types)]
   HU,
   #[allow(non_camel_case_types)]
   ID,
   #[allow(non_camel_case_types)]
   IE,
   #[allow(non_camel_case_types)]
   IL,
   #[allow(non_camel_case_types)]
   IM,
   #[allow(non_camel_case_types)]
   IN,
   #[allow(non_camel_case_types)]
   IO,
   #[allow(non_camel_case_types)]
   IQ,
   #[allow(non_camel_case_types)]
   IR,
   #[allow(non_camel_case_types)]
   IS,
   #[allow(non_camel_case_types)]
   IT,
   #[allow(non_camel_case_types)]
   JE,
   #[allow(non_camel_case_types)]
   JM,
   #[allow(non_camel_case_types)]
   JO,
   #[allow(non_camel_case_types)]
   JP,
   #[allow(non_camel_case_types)]
   KE,
   #[allow(non_camel_case_types)]
   KG,
   #[allow(non_camel_case_types)]
   KH,
   #[allow(non_camel_case_types)]
   KI,
   #[allow(non_camel_case_types)]
   KM,
   #[allow(non_camel_case_types)]
   KN,
   #[allow(non_camel_case_types)]
   KP,
   #[allow(non_camel_case_types)]
   KR,
   #[allow(non_camel_case_types)]
   KW,
   #[allow(non_camel_case_types)]
   KY,
   #[allow(non_camel_case_types)]
   KZ,
   #[allow(non_camel_case_types)]
   LA,
   #[allow(non_camel_case_types)]
   LB,
   #[allow(non_camel_case_types)]
   LC,
   #[allow(non_camel_case_types)]
   LI,
   #[allow(non_camel_case_types)]
   LK,
   #[allow(non_camel_case_types)]
   LR,
   #[allow(non_camel_case_types)]
   LS,
   #[allow(non_camel_case_types)]
   LT,
   #[allow(non_camel_case_types)]
   LU,
   #[allow(non_camel_case_types)]
   LV,
   #[allow(non_camel_case_types)]
   LY,
   #[allow(non_camel_case_types)]
   MA,
   #[allow(non_camel_case_types)]
   MC,
   #[allow(non_camel_case_types)]
   MD,
   #[allow(non_camel_case_types)]
   ME,
   #[allow(non_camel_case_types)]
   MF,
   #[allow(non_camel_case_types)]
   MG,
   #[allow(non_camel_case_types)]
   MH,
   #[allow(non_camel_case_types)]
   MK,
   #[allow(non_camel_case_types)]
   ML,
   #[allow(non_camel_case_types)]
   MM,
   #[allow(non_camel_case_types)]
   MN,
   #[allow(non_camel_case_types)]
   MO,
   #[allow(non_camel_case_types)]
   MP,
   #[allow(non_camel_case_types)]
   MQ,
   #[allow(non_camel_case_types)]
   MR,
   #[allow(non_camel_case_types)]
   MS,
   #[allow(non_camel_case_types)]
   MT,
   #[allow(non_camel_case_types)]
   MU,
   #[allow(non_camel_case_types)]
   MV,
   #[allow(non_camel_case_types)]
   MW,
   #[allow(non_camel_case_types)]
   MX,
   #[allow(non_camel_case_types)]
   MY,
   #[allow(non_camel_case_types)]
   MZ,
   #[allow(non_camel_case_types)]
   NA,
   #[allow(non_camel_case_types)]
   NC,
   #[allow(non_camel_case_types)]
   NE,
   #[allow(non_camel_case_types)]
   NF,
   #[allow(non_camel_case_types)]
   NG,
   #[allow(non_camel_case_types)]
   NI,
   #[allow(non_camel_case_types)]
   NL,
   #[allow(non_camel_case_types)]
   NO,
   #[allow(non_camel_case_types)]
   NP,
   #[allow(non_camel_case_types)]
   NR,
   #[allow(non_camel_case_types)]
   NU,
   #[allow(non_camel_case_types)]
   NZ,
   #[allow(non_camel_case_types)]
   OM,
   #[allow(non_camel_case_types)]
   PA,
   #[allow(non_camel_case_types)]
   PE,
   #[allow(non_camel_case_types)]
   PF,
   #[allow(non_camel_case_types)]
   PG,
   #[allow(non_camel_case_types)]
   PH,
   #[allow(non_camel_case_types)]
   PK,
   #[allow(non_camel_case_types)]
   PL,
   #[allow(non_camel_case_types)]
   PM,
   #[allow(non_camel_case_types)]
   PN,
   #[allow(non_camel_case_types)]
   PR,
   #[allow(non_camel_case_types)]
   PS,
   #[allow(non_camel_case_types)]
   PT,
   #[allow(non_camel_case_types)]
   PW,
   #[allow(non_camel_case_types)]
   PY,
   #[allow(non_camel_case_types)]
   QA,
   #[allow(non_camel_case_types)]
   RE,
   #[allow(non_camel_case_types)]
   RO,
   #[allow(non_camel_case_types)]
   RS,
   #[allow(non_camel_case_types)]
   RU,
   #[allow(non_camel_case_types)]
   RW,
   #[allow(non_camel_case_types)]
   SA,
   #[allow(non_camel_case_types)]
   SB,
   #[allow(non_camel_case_types)]
   SC,
   #[allow(non_camel_case_types)]
   SD,
   #[allow(non_camel_case_types)]
   SE,
   #[allow(non_camel_case_types)]
   SG,
   #[allow(non_camel_case_types)]
   SH,
   #[allow(non_camel_case_types)]
   SI,
   #[allow(non_camel_case_types)]
   SJ,
   #[allow(non_camel_case_types)]
   SK,
   #[allow(non_camel_case_types)]
   SL,
   #[allow(non_camel_case_types)]
   SM,
   #[allow(non_camel_case_types)]
   SN,
   #[allow(non_camel_case_types)]
   SO,
   #[allow(non_camel_case_types)]
   SR,
   #[allow(non_camel_case_types)]
   SS,
   #[allow(non_camel_case_types)]
   ST,
   #[allow(non_camel_case_types)]
   SV,
   #[allow(non_camel_case_types)]
   SX,
   #[allow(non_camel_case_types)]
   SY,
   #[allow(non_camel_case_types)]
   SZ,
   #[allow(non_camel_case_types)]
   TC,
   #[allow(non_camel_case_types)]
   TD,
   #[allow(non_camel_case_types)]
   TF,
   #[allow(non_camel_case_types)]
   TG,
   #[allow(non_camel_case_types)]
   TH,
   #[allow(non_camel_case_types)]
   TJ,
   #[allow(non_camel_case_types)]
   TK,
   #[allow(non_camel_case_types)]
   TL,
   #[allow(non_camel_case_types)]
   TM,
   #[allow(non_camel_case_types)]
   TN,
   #[allow(non_camel_case_types)]
   TO,
   #[allow(non_camel_case_types)]
   TR,
   #[allow(non_camel_case_types)]
   TT,
   #[allow(non_camel_case_types)]
   TV,
   #[allow(non_camel_case_types)]
   TW,
   #[allow(non_camel_case_types)]
   TZ,
   #[allow(non_camel_case_types)]
   UA,
   #[allow(non_camel_case_types)]
   UG,
   #[allow(non_camel_case_types)]
   UM,
   #[allow(non_camel_case_types)]
   US,
   #[allow(non_camel_case_types)]
   UY,
   #[allow(non_camel_case_types)]
   UZ,
   #[allow(non_camel_case_types)]
   VA,
   #[allow(non_camel_case_types)]
   VC,
   #[allow(non_camel_case_types)]
   VE,
   #[allow(non_camel_case_types)]
   VG,
   #[allow(non_camel_case_types)]
   VI,
   #[allow(non_camel_case_types)]
   VN,
   #[allow(non_camel_case_types)]
   VU,
   #[allow(non_camel_case_types)]
   WF,
   #[allow(non_camel_case_types)]
   WS,
   #[allow(non_camel_case_types)]
   YE,
   #[allow(non_camel_case_types)]
   YT,
   #[allow(non_camel_case_types)]
   ZA,
   #[allow(non_camel_case_types)]
   ZM,
   #[allow(non_camel_case_types)]
   ZW,
}

