use color_eyre::eyre::Result;
use rocket::serde::Deserialize;

macro_rules! from_env {
    ($varname:expr) => {
        std::env::var($varname)
            .expect(format!("{} must be set!", $varname).as_str())
    };
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct APVariables {
    pub endpoint: String,
    pub project: String,
    pub api_key: String,
}

impl APVariables {
    pub async fn check_session(
        &self,
        session_id: &str,
        user_id: &str,
    ) -> Result<bool> {
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}/sessions", self.endpoint, user_id);
        let response = client
            .get(url)
            .header("X-Appwrite-Key", self.api_key.clone())
            .header("X-Appwrite-Project", self.project.clone())
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<UserSessions>()
            .await?;
        Ok(response.sessions.iter().any(|s| s.id == session_id))
    }
}

impl Default for APVariables {
    fn default() -> Self {
        Self {
            endpoint: from_env!("APPWRITE_ENDPOINT"),
            project: from_env!("APPWRITE_PROJECT"),
            api_key: from_env!("APPWRITE_API_KEY"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
struct UserSessions {
    total: i64,
    sessions: Vec<Sessions>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
struct Sessions {
    #[serde(rename = "$id")]
    id: String,
    #[serde(rename = "$createdAt")]
    created_at: String,
    user_id: String,
    expire: String,
    provider: String,
    provider_uid: String,
    provider_access_token: String,
    provider_access_token_expiry: String,
    provider_refresh_token: String,
    ip: String,
    os_code: String,
    os_name: String,
    os_version: String,
    client_type: String,
    client_code: String,
    client_name: String,
    client_version: String,
    client_engine: String,
    client_engine_version: String,
    device_name: String,
    device_brand: String,
    device_model: String,
    country_code: String,
    country_name: String,
    current: bool,
}
