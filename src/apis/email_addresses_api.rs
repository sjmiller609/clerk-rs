/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::Error;
use crate::{apis::ResponseContent, clerk::Clerk};

/// struct for typed errors of method [`create_email_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmailAddressError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_email_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEmailAddressError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_email_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEmailAddressError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_email_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEmailAddressError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

pub struct EmailAddresses;

impl EmailAddresses {
	/// Create a new email address
	pub async fn create_email_address(
		clerk_client: &Clerk,
		create_email_address_request: Option<crate::models::CreateEmailAddressRequest>,
	) -> Result<crate::models::EmailAddress, Error<CreateEmailAddressError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/email_addresses", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&create_email_address_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateEmailAddressError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete the email address with the given ID
	pub async fn delete_email_address(
		clerk_client: &Clerk,
		email_address_id: &str,
	) -> Result<crate::models::DeletedObject, Error<DeleteEmailAddressError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/email_addresses/{email_address_id}",
			local_var_configuration.base_path,
			email_address_id = crate::apis::urlencode(email_address_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<DeleteEmailAddressError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns the details of an email address.
	pub async fn get_email_address(clerk_client: &Clerk, email_address_id: &str) -> Result<crate::models::EmailAddress, Error<GetEmailAddressError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/email_addresses/{email_address_id}",
			local_var_configuration.base_path,
			email_address_id = crate::apis::urlencode(email_address_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetEmailAddressError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Updates an email address.
	pub async fn update_email_address(
		clerk_client: &Clerk,
		email_address_id: &str,
		update_email_address_request: Option<crate::models::UpdateEmailAddressRequest>,
	) -> Result<crate::models::EmailAddress, Error<UpdateEmailAddressError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/email_addresses/{email_address_id}",
			local_var_configuration.base_path,
			email_address_id = crate::apis::urlencode(email_address_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&update_email_address_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateEmailAddressError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}