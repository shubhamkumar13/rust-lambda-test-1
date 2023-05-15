use std::{cmp::Ordering, collections::HashMap, process::Command, str::FromStr};

use lambda_runtime::LambdaEvent;
use lib::{
    org_accordproject_acceptanceofdelivery_0_15_0::{
        AcceptanceOfDeliveryClause, InspectDeliverable, InspectionResponse, InspectionStatus,
    },
    org_accordproject_organization_0_2_0::Organization,
    org_accordproject_runtime_0_2_0::*,
    utils::*,
};

use chrono::offset::Utc;
use chrono::{naive::Days, DateTime};
use lambda_http::{Body, Error};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, json, Value};

pub fn trim(s: &Value) -> String {
    let Some(s) = s.as_str() else {
        panic!("cannot convert JSON field value to string")
    };

    s.trim_matches('\"').to_string()
}

pub struct AcceptanceOfDeliveryClauseImpl(pub AcceptanceOfDeliveryClause);
pub struct OrganizationImpl(pub Organization);

#[derive(Debug, Deserialize, Serialize)]
pub struct InspectDeliverableImpl(pub InspectDeliverable);
pub struct InspectionResponseImpl(pub InspectionResponse);

impl OrganizationImpl {
    pub fn new(field_name: &'static str, field_value: String) -> Self {
        let organization = r#"
            {
                "$class": "org.accordproject.organization.Organization",
                "identifier": "resource1",
                "$identifier": "resource1"
            }
        "#;
        let organization: HashMap<&str, &str> = from_str(&organization).unwrap();

        let Some(identifier) = field_value
            .split("#")
            .last()
            .and_then(|s| {
                Some(s.replace("%20", " ").trim_matches('\"').to_string())
            }) else {
                panic!("Cannot find the field value")
            };

        Self(Organization {
            _class: organization[&"$class"].to_string(),
            identifier: identifier.clone(),
            name: Some(field_name.to_string()),
            description: None,
            duns: None,
            place: None,
            _identifier: identifier,
        })
    }
}

impl AcceptanceOfDeliveryClauseImpl {
    pub fn from_value(contract: Value) -> Result<Self, Error> {
        let contract_map = contract
            .as_object()
            .ok_or::<Error>("Cannot convert Acceptance of Delivery Clause to HashMap".into())?;

        let shipper = OrganizationImpl::new("shipper", contract_map["shipper"].to_string()).0;

        let receiver = OrganizationImpl::new("receiver", contract_map["receiver"].to_string()).0;

        let business_days = contract_map["businessDays"]
            .as_u64()
            .ok_or::<Error>("cannot convert business days to u64".into())?;

        Ok(Self(AcceptanceOfDeliveryClause {
            _class: trim(&contract_map["$class"]),
            shipper,
            receiver,
            deliverable: trim(&contract_map["deliverable"]),
            business_days,
            attachment: trim(&contract_map["attachment"]),
            clause_id: trim(&contract_map["clauseId"]),
            _identifier: trim(&contract_map["$identifier"]),
        }))
    }

    pub fn from_str(s: &'static str) -> Result<Self, Error> {
        let s = serde_json::from_str(s)?;
        Self::from_value(s)
    }
}

impl<'a> InspectDeliverableImpl {
    pub fn new(
        _class: String,
        deliverable_received_at: DateTime<Utc>,
        inspection_passed: bool,
        _timestamp: DateTime<Utc>,
    ) -> Self {
        Self(InspectDeliverable {
            _class,
            deliverable_received_at,
            inspection_passed,
            _timestamp,
        })
    }

    pub fn from_value(request: Value) -> Result<Self, Error> {
        let deliverable = request["body"]
            .as_object()
            .ok_or("Cannot convert to Map Object")?;

        let deliverable_received_at: DateTime<Utc> =
            serde_json::from_value(deliverable["deliverableReceivedAt"].clone())?;

        let inspection_passed = deliverable["inspectionPassed"]
            .as_bool()
            .ok_or("inspectionPassed field empty")?;

        Ok(Self::new(
            trim(&deliverable["$class"]),
            deliverable_received_at,
            inspection_passed,
            Utc::now(),
        ))
    }

    pub fn from_str(s: &'a str) -> Result<Self, Error> {
        let deliverable: Value = serde_json::from_str(s)?;
        Self::from_value(deliverable)
    }
}

impl InspectionResponseImpl {
    pub fn new(
        status: InspectionStatus,
        shipper: Organization,
        receiver: Organization,
        timestamp: DateTime<Utc>,
    ) -> InspectionResponseImpl {
        Self(InspectionResponse {
            _class: "org.accordproject.acceptanceofdelivery.InspectionResponse".into(),
            status,
            shipper,
            receiver,
            _timestamp: timestamp, // this is not ideal
        })
    }

    pub fn to_str(self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.0)
    }
}

fn now(s: &'static str) -> DateTime<Utc> {
    DateTime::from_str(s).unwrap_or_else(|_| panic!("Cannot convert string to UTC datetime format"))
}

const CONTRACT_STR: &str = r#"
    {
        "$class": "org.accordproject.acceptanceofdelivery.AcceptanceOfDeliveryClause",
        "shipper": "resource:org.accordproject.organization.Organization#Party%20A",
        "receiver": "resource:org.accordproject.organization.Organization#Party%20B",
        "deliverable": "Widgets",
        "businessDays": 10,
        "attachment": "Attachment X",
        "clauseId": "16ddc768-3847-4fc6-a7e4-9b7d94c020dd",
        "$identifier": "16ddc768-3847-4fc6-a7e4-9b7d94c020dd"
    }
"#;

const DELIVERABLE: &str = r#"
    {
        "$class": "org.accordproject.acceptanceofdelivery.InspectDeliverable",
        "deliverableReceivedAt": "2019-01-22 03:24:00Z",
        "inspectionPassed": true
    }
"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    request: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    // #[serde(rename = "isBase64Encoded")]
    // is_base64_encoded: bool,
    // #[serde(rename = "statusCode")]
    // status_code: i32,
    // headers: Value,
    response: String,
}

impl Response {
    pub fn new(input: InspectionResponse) -> Result<Self, serde_json::Error> {
        let response = serde_json::to_string(&input)?;
        Ok(Response { response })
    }
}

pub async fn function_handler(request: LambdaEvent<Value>) -> Result<Value, Error> {
    // timestamp is just for testing the template
    let now = now("2019-01-31T16:34:00-05:00");
    let contract = AcceptanceOfDeliveryClauseImpl::from_str(CONTRACT_STR)?.0;
    eprintln!("Body = {:#?}", request.payload["body"]);
    let Value::String(body) = request.payload["body"].clone() else {
        return Err("Body is not a string".into())
    };
    // eprintln!("Request = {:#?}", serde_json::from_str::<Request>(&body));
    let request: Request = serde_json::from_str(&body)?;
    let request = request.request;
    eprintln!("Request = {:#?}", request);
    let received: DateTime<Utc> = serde_json::from_value(request["deliverableReceivedAt"].clone())?;
    let inspection_passed: bool = serde_json::from_value(request["inspectionPassed"].clone())?;
    if now < received {
        return Err("Transaction time is before the deliverable date.".into());
    };

    let status: InspectionStatus = {
        match received.checked_add_days(Days::new(contract.business_days)) {
            None => Err("This is not a valid date"),
            Some(date) => Ok({
                if now < date {
                    if inspection_passed {
                        InspectionStatus::PASSED_TESTING
                    } else {
                        InspectionStatus::FAILED_TESTING
                    }
                } else {
                    InspectionStatus::OUTSIDE_INSPECTION_PERIOD
                }
            }),
        }
    }?;

    let response = Response::new(
        InspectionResponseImpl::new(status, contract.shipper, contract.receiver, now).0,
    )?
    .response;

    Ok(json!({
        "isBase64Encoded" : false,
        "statusCode" : 200,
        "headers" : { },
        "body" : response,
    }))
}
