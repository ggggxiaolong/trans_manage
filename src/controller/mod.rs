use async_graphql::*;
use crate::service::SYS_USER_SERVICE;

pub struct Query ;

#[Object]
impl Query {
    #[field(desc = "Returns the sum of a and b")]
    //-> FieldResult<Token>
    async fn login(&self, context: &Context<'_>, mail: String, password: String) -> i32  {
        // match user::table.filter(user::mail.eq(mail)).load::<User>(&conn) {
        //     Ok(mut users) => {
        //         if let Some(mut user) = users.pop() {
        //             match bcrypt::verify(&password, &user.password) {
        //                 Ok(true) => {
        //                     user.remove_password();
        //                     Ok(crate::app::token::gen_user_token(user))
        //                 }
        //                 Ok(false) => Err(CustomError::MailOrPasswordFail),
        //                 Err(e) => Err(CustomError::Internal(format!("{:?}", e))),
        //             }
        //         } else {
        //             Err(CustomError::MailOrPasswordFail)
        //         }
        //     }
        //     Err(e) => Err(CustomError::Internal(format!("{:?}", e))),
        // }
        0
    }

    async fn borrow_from_context_data<'ctx>(
        &self,
        ctx: &'ctx Context<'_>
    ) -> FieldResult<&'ctx String> {
        ctx.data::<String>()
    }
}

pub fn gen_schema()-> async_graphql::Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::new(Query, EmptyMutation, EmptySubscription)
}