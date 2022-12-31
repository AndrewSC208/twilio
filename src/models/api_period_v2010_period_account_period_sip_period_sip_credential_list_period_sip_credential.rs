/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.38.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential {
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The unique id of the Account that is responsible for this resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The unique id that identifies the credential list that includes this credential
    #[serde(rename = "credential_list_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub credential_list_sid: Option<Option<String>>,
    /// The username for this credential.
    #[serde(rename = "username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
    /// The date that this resource was created, given as GMT in RFC 2822 format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date that this resource was last updated, given as GMT in RFC 2822 format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The URI for this resource, relative to https://api.twilio.com
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential {
        ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential {
            sid: None,
            account_sid: None,
            credential_list_sid: None,
            username: None,
            date_created: None,
            date_updated: None,
            uri: None,
        }
    }
}


