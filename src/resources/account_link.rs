// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::AccountId;
use crate::params::{Expand, Object, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "AccountLink".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The timestamp at which this account link will expire.
    pub expires_at: Timestamp,

    /// The URL for the account link.
    pub url: String,
}

impl AccountLink {
    /// Creates an AccountLink object that returns a single-use Stripe URL that the user can redirect their user to in order to take them through the Connect Onboarding flow.
    pub fn create(client: &Client, params: CreateAccountLink<'_>) -> Response<AccountLink> {
        client.post_form("/account_links", &params)
    }
}

impl Object for AccountLink {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "account_link"
    }
}

/// The parameters for `AccountLink::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateAccountLink<'a> {
    /// The identifier of the account to create an account link for.
    pub account: AccountId,

    /// Which information the platform needs to collect from the user.
    ///
    /// One of `currently_due` or `eventually_due`.
    /// Default is `currently_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect: Option<AccountLinkCollect>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL that the user will be redirected to if the account link is no longer valid.
    pub failure_url: &'a str,

    /// The URL that the user will be redirected to upon leaving or completing the linked flow successfully.
    pub success_url: &'a str,

    /// The type of account link the user is requesting.
    ///
    /// Possible values are `custom_account_verification` or `custom_account_update`.
    #[serde(rename = "type")]
    pub type_: AccountLinkType,
}

impl<'a> CreateAccountLink<'a> {
    pub fn new(
        account: AccountId,
        failure_url: &'a str,
        success_url: &'a str,
        type_: AccountLinkType,
    ) -> Self {
        CreateAccountLink {
            account,
            collect: Default::default(),
            expand: Default::default(),
            failure_url,
            success_url,
            type_,
        }
    }
}

/// An enum representing the possible values of an `CreateAccountLink`'s `collect` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountLinkCollect {
    CurrentlyDue,
    EventuallyDue,
}

impl AccountLinkCollect {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountLinkCollect::CurrentlyDue => "currently_due",
            AccountLinkCollect::EventuallyDue => "eventually_due",
        }
    }
}

impl AsRef<str> for AccountLinkCollect {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountLinkCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateAccountLink`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountLinkType {
    CustomAccountUpdate,
    CustomAccountVerification,
}

impl AccountLinkType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountLinkType::CustomAccountUpdate => "custom_account_update",
            AccountLinkType::CustomAccountVerification => "custom_account_verification",
        }
    }
}

impl AsRef<str> for AccountLinkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
