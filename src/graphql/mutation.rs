use super::Context;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn api_version(
        context: &Context,
    ) -> String {
        if context.user_auth {
            "0.1 (authentified)"
        } else {
            "0.1 (not authentified)"
        }.into()
    }
}
