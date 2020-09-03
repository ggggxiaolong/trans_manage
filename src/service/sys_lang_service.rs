pub struct SysLangService {}
impl SysLangService {
    pub async fn page_language(
        page: i32,
        size: i32,
        project_id: i32,
        search: Option<String>
    ){}
    pub async fn trans(en: &str, to: &str){}
    pub async fn add(){}
    pub async fn update(){}
    pub async fn update_multiple(){}
}