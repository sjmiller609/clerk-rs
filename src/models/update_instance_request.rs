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
pub struct UpdateInstanceRequest {
	/// Toggles test mode for this instance, allowing the use of test email addresses and phone numbers. Defaults to true for development instances.
	#[serde(
		rename = "test_mode",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub test_mode: Option<Option<bool>>,
	/// Whether the instance should be using the HIBP service to check passwords for breaches
	#[serde(
		rename = "hibp",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub hibp: Option<Option<bool>>,
	/// The \"enhanced_email_deliverability\" feature will send emails from \"verifications@clerk.dev\" instead of your domain. This can be helpful if you do not have a high domain reputation.
	#[serde(
		rename = "enhanced_email_deliverability",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub enhanced_email_deliverability: Option<Option<bool>>,
	#[serde(
		rename = "support_email",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub support_email: Option<Option<String>>,
	#[serde(
		rename = "clerk_js_version",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub clerk_js_version: Option<Option<String>>,
	#[serde(rename = "experimental_allowed_origins", skip_serializing_if = "Option::is_none")]
	pub experimental_allowed_origins: Option<Vec<String>>,
	#[serde(rename = "allowed_origins", skip_serializing_if = "Option::is_none")]
	pub allowed_origins: Option<Vec<String>>,
	/// Whether the instance should operate in cookieless development mode (i.e. without third-party cookies). Deprecated: Please use `url_based_session_syncing` instead.
	#[serde(rename = "cookieless_dev", skip_serializing_if = "Option::is_none")]
	pub cookieless_dev: Option<bool>,
	/// Whether the instance should use URL-based session syncing in development mode (i.e. without third-party cookies).
	#[serde(rename = "url_based_session_syncing", skip_serializing_if = "Option::is_none")]
	pub url_based_session_syncing: Option<bool>,
}

impl UpdateInstanceRequest {
	pub fn new() -> UpdateInstanceRequest {
		UpdateInstanceRequest {
			test_mode: None,
			hibp: None,
			enhanced_email_deliverability: None,
			support_email: None,
			clerk_js_version: None,
			experimental_allowed_origins: None,
			allowed_origins: None,
			cookieless_dev: None,
			url_based_session_syncing: None,
		}
	}
}