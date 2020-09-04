
pub struct SysUserService {}

impl SysUserService {
    pub async fn login(mail: String, pass: String){}
    pub async fn change_pass(mail: String, old_pass: String, new_pass: String){}
    pub async fn refresh_token(token: String){}
}