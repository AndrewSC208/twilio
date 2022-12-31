/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.38.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncomingPhoneNumberMobileEnumEmergencyStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,

}

impl ToString for IncomingPhoneNumberMobileEnumEmergencyStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("Active"),
            Self::Inactive => String::from("Inactive"),
        }
    }
}

impl Default for IncomingPhoneNumberMobileEnumEmergencyStatus {
    fn default() -> IncomingPhoneNumberMobileEnumEmergencyStatus {
        Self::Active
    }
}




