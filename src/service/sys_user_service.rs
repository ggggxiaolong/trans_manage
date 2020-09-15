
pub struct SysUserService {}

impl SysUserService {
    pub async fn login(&self, mail: String, pass: String){}
    pub async fn change_pass(&self, mail: String, old_pass: String, new_pass: String){}
    pub async fn refresh_token(&self, token: String){}
}