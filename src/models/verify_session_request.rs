/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifySessionRequest {
	/// The JWT that is sent via the `__session` cookie from your frontend. Note: this JWT must be associated with the supplied session ID.
	#[serde(rename = "token", skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
}

impl VerifySessionRequest {
	pub fn new() -> VerifySessionRequest {
		VerifySessionRequest { token: None }
	}
}