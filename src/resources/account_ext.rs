use crate::{
    config::{Client, Response},
    Account, AccountId, LoginLink,
};
use serde_derive::{Deserialize, Serialize};

impl Account {
    /// Creates an account login link.
    ///
    /// For more details see [https://stripe.com/docs/api/account/create_login_link](https://stripe.com/docs/api/account/create_login_link).
    pub fn create_login_link(
        client: &Client,
        account_id: &AccountId,
        params: CreateLoginLink,
    ) -> Response<LoginLink> {
        client.post_form(&format!("/accounts/{}/login_links", account_id), params)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateLoginLink {
    /// Where to redirect the user after they log out of their dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
