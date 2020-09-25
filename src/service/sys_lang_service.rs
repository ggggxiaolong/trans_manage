pub struct SysLangService {}
impl SysLangService {
    pub async fn page_language(
        &self,
        page: i32,
        size: i32,
        project_id: i32,
        search: Option<String>
    ){

    }
    pub async fn trans(&self, en: &str, to: &str){}
    pub async fn add(&self, ){}
    pub async fn update(&self,){}
    pub async fn update_multiple(&self,){}
}