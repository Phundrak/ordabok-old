use crate::appwrite::APVariables;
use crate::db::Database;

use tracing::info;

macro_rules! from_env {
    ($varname:expr) => {
        std::env::var($varname)
            .expect(format!("{} must be set!", $varname).as_str())
    };
}

#[derive(Debug, Clone)]
pub struct OtherEnvVar {
    pub admin_key: String,
}

impl Default for OtherEnvVar {
    fn default() -> Self {
        Self {
            admin_key: from_env!("ADMIN_KEY"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Context {
    pub db: Database,
    pub appwrite: APVariables,
    pub user_auth: Option<String>,
    pub other_vars: OtherEnvVar,
}

impl Context {
    /// Check if a request is performed by an autentificated user.
    ///
    /// The HTTP header `Authorization` must be a single string in the
    /// form `userId;userSessionId` with `userId` and `userSessionId`
    /// being variables given by Appwrite to users that are logged in.
    ///
    /// The function returns either the user's ID if the user is
    /// authentified or `None`.
    pub async fn user_auth<'r>(
        &self,
        auth_token: Option<&'r str>,
    ) -> Option<String> {
        if let Some(token) = auth_token {
            let key = token.split(';').collect::<Vec<_>>();
            if key.len() == 2 {
                let user_id = key[0];
                let session_id = key[1];
                match self.appwrite.check_session(session_id, user_id).await {
                    Ok(true) => Some(key[0].to_string()),
                    Ok(false) => None,
                    Err(e) => {
                        info!("Error checking user session: {:?}", e);
                        None
                    }
                }
            } else {
                info!("Invalid session key: {}", token);
                None
            }
        } else {
            None
        }
    }

    pub async fn attach_auth<'r>(&self, auth_token: Option<&'r str>) -> Self {
        let mut res = self.clone();
        res.user_auth = self.user_auth(auth_token).await;
        res
    }
}

impl juniper::Context for Context {}
