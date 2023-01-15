use crate::appwrite::APVariables;
use crate::db::Database;

use tracing::info;

#[derive(Default, Debug, Clone)]
pub struct Context {
    pub db: Database,
    pub appwrite: APVariables,
    pub user_auth: bool,
}

impl Context {
    /// HTTP header for a user's session
    ///
    /// This header `Authorization` must be a single string in the
    /// form `userId;userSessionId` with `userId` and `userSessionId`
    /// being variables given by Appwrite to users that are logged in.
    pub async fn user_auth<'r>(&self, auth_token: Option<&'r str>) -> bool {
        if let Some(token) = auth_token {
            let key = token.split(';').collect::<Vec<_>>();
            if key.len() == 2 {
                let user_id = key[0];
                let session_id = key[1];
                match self.appwrite.check_session(session_id, user_id).await {
                    Ok(val) => val,
                    Err(e) => {
                        info!("Error checking user session: {:?}", e);
                        false
                    }
                }
            } else {
                info!("Invalid session key: {}", token);
                false
            }
        } else {
            false
        }
    }

    pub async fn attach_auth<'r>(&self, auth_token: Option<&'r str>) -> Self {
        let mut res = self.clone();
        res.user_auth = self.user_auth(auth_token).await;
        res
    }

}

impl juniper::Context for Context {}
