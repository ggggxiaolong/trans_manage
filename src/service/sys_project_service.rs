use crate::dao::DBPool;
use crate::domain::domain::Project;
use crate::domain::vo::VOProject;
use sqlx::{Error, Acquire};

pub struct SysProjectService {}

impl SysProjectService {
    pub async fn all_project(&self) -> Result<Vec<VOProject>, Error> {
        let projects: Vec<Project> = sqlx::query_as("SELECT id,name FROM project").fetch_all(DBPool::get_pool()).await?;
        Ok(projects.into_iter().map(|p| p.into()).collect())
    }

    pub async fn mergeUpdate(&self, project_id: i32, user_id: i32) -> Result<i32, Error> {
        // transaction.co
        let mut tx = DBPool::get_pool().begin().await?;
        sqlx::query("delete from tem_lang").execute(&mut tx).await;
        sqlx::query("insert into tem_lang select id, new_en, new_ja, new_ko, new_sk, new_cs, new_fr, new_es, new_pt, new_not_trans, new_descripe, new_label, new_file_name, new_mode_name, new_project_id from lang where project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set en = (select new_en from tem_lang where tem_lang.id = lang.id) where new_en is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set ja = (select new_ja from tem_lang where tem_lang.id = lang.id) where new_ja is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set ko = (select new_ko from tem_lang where tem_lang.id = lang.id) where new_ko is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set sk = (select new_sk from tem_lang where tem_lang.id = lang.id) where new_sk is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set cs = (select new_cs from tem_lang where tem_lang.id = lang.id) where new_cs is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set fr = (select new_fr from tem_lang where tem_lang.id = lang.id) where new_fr is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set es = (select new_es from tem_lang where tem_lang.id = lang.id) where new_es is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set pt = (select new_pt from tem_lang where tem_lang.id = lang.id) where new_pt is not null and status != 0 and project_id = {$1}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set user_id = {$1} where status != 0 and project_id = {$2}").bind(user_id).bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set not_trans = (select new_not_trans from tem_lang where tem_lang.id = lang.id) where new_not_trans is not null and status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set descripe = (select new_descripe from tem_lang where tem_lang.id = lang.id) where new_descripe is not null and status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set label = (select new_label from tem_lang where tem_lang.id = lang.id) where new_label is not null and status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set file_name = (select new_file_name from tem_lang where tem_lang.id = lang.id) where new_file_name is not null and status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set mode_name = (select new_mode_name from tem_lang where tem_lang.id = lang.id) where new_mode_name is not null and status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set project_id = (select new_project_id from tem_lang where tem_lang.id = lang.id) where new_project_id is not null and status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        sqlx::query("update lang set new_user_id = null, new_en=null, new_ja=null, new_ko=null, new_sk=null, new_cs=null, new_fr=null, new_es=null, new_pt=null, new_not_trans=null, new_descripe=null, new_label=null, new_file_name=null, new_mode_name=null, new_project_id=null, status = 0, update_time = CURRENT_TIMESTAMP where status != 0 and project_id = {}").bind(project_id).execute(&mut tx).await;
        tx.commit();
        Ok(project_id)
    }
}