use super::Context;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn api_version(
        session_id: Option<String>,
        user_id: Option<String>,
        context: &Context,
    ) -> String {
        "0.1".into()
        // if session_id.is_some() && user_id.is_some() {
        //     match context
        //         .appwrite
        //         .check_session(session_id.unwrap(), user_id.unwrap())
        //     {
        //         Ok(true) => "0.1 (authentified)".into(),
        //         Ok(false) => "0.1 (not authentified)".into(),
        //         Err(e) => {
        //             info!(
        //                 "Error while checking if the user is connected: {:?}",
        //                 e
        //             );
        //             "0.1 (auth failed)"
        //         }
        //     }
        // } else {
        //     "0.1 (not authentified)"
        // }
    }
}
