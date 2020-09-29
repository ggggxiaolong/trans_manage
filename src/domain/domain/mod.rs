use chrono::NaiveDateTime;
use crate::domain::dto::LanguageSearchType;
use crate::utils::sql_helper::SqlHelper;

/// 数据库对象
#[derive(sqlx::FromRow)]
pub struct Lang {
    pub id: i32,
    pub user_id: i32,
    pub en: String,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    pub pt: Option<String>,
    pub not_trans: i32,
    pub descripe: Option<String>,
    pub label: Option<String>,
    pub file_name: Option<String>,
    pub mode_name: Option<String>,
    pub project_id: i32,
    pub new_user_id: Option<i32>,
    pub new_en: Option<String>,
    pub new_ja: Option<String>,
    pub new_ko: Option<String>,
    pub new_sk: Option<String>,
    pub new_cs: Option<String>,
    pub new_fr: Option<String>,
    pub new_es: Option<String>,
    pub new_pt: Option<String>,
    pub new_not_trans: Option<i32>,
    pub new_descripe: Option<String>,
    pub new_label: Option<String>,
    pub new_file_name: Option<String>,
    pub new_mode_name: Option<String>,
    pub new_project_id: Option<i32>,
    pub status: i32,
    //0 为最新， 1为更新， 2为新增
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
    pub update_time: NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

pub mod sql {
    pub mod user {
        use crate::dao::DBPool;
        use sqlx::{Error};
        use crate::domain::domain::User;
        use sqlx::sqlite::SqliteDone;
        use chrono::Utc;

        pub async fn query_by_mail(mail: String) -> Result<Option<User>, Error> {
            sqlx::query_as("SELECT id,username,mail,password,update_time FROM user WHERE mail = ?").bind(mail).fetch_optional(DBPool::get_pool()).await
        }

        pub async fn update_password(id: i32, password: String) -> Result<SqliteDone, Error> {
            sqlx::query("UPDATE user SET password = ?,update_time = ?  WHERE id = ?").bind(password).bind(Utc::now().naive_utc()).bind(id).execute(DBPool::get_pool()).await
        }

        pub async fn query_by_id(id: i32)-> Result<Option<User>, Error> {
            sqlx::query_as("SELECT id,username,mail,password,update_time FROM user WHERE id = ?").bind(id).fetch_optional(DBPool::get_pool()).await
        }
    }

    pub mod project {
        use crate::domain::domain::Project;
        use crate::dao::DBPool;
        use sqlx::{Error};
        lazy_static! {
            static ref SQL_TABLE_NAME: String = String::from("project");
            static ref SQL_FETCH_ALL: String = String::from("SELECT id,name FROM project");
        }

        pub async fn query_all() -> Result<Vec<Project>, Error>{
            sqlx::query_as(&SQL_FETCH_ALL).fetch_all(DBPool::get_pool()).await
        }
    }

    pub mod lang {
        use crate::utils::sql_helper::SqlHelper;
        use crate::domain::dto::LanguageSearchType;
        use crate::domain::domain::Lang;
        use sqlx::{Error};
        use crate::dao::DBPool;
        lazy_static! {
            static ref SQL_TABLE_NAME: String = String::from("lang");
            static ref SQL_TABLE_COLUMNS: String = "id,user_id,en,ja,ko,sk,cs,fr,es,pt,not_trans,descripe,label,file_name,mode_name,project_id,new_user_id,new_en,new_ja,new_ko,new_sk,new_cs,new_fr,new_es,new_pt,new_not_trans,new_descripe,new_label,new_file_name,new_mode_name,new_project_id,status,create_time,update_time".to_string();
        }
        pub async fn page_fetch(page: i32, page_size: i32, search: Option<String>, project_id: i32, status: LanguageSearchType) -> Result<Vec<Lang>, Error> {
            let helper: SqlHelper = SqlHelper::query(&SQL_TABLE_NAME, &SQL_TABLE_COLUMNS);
            let sql = status.to_where_sql(helper)
                .and_where_eq("project_id", &project_id.to_string())
                .and_where_like_option("en", search)
                .page(page, page_size).sql();
            sqlx::query_as(&sql).fetch_all(DBPool::get_pool()).await
        }
    }
}

impl LanguageSearchType {
    pub fn to_where_sql(self, mut helper: SqlHelper) -> SqlHelper {
        //0 为最新， 1为更新， 2为新增
        match self {
            LanguageSearchType::All => {}
            LanguageSearchType::New => {
                helper.and_where_eq("status", "2");
            }
            LanguageSearchType::Update => {
                helper.and_where_eq("status", "1");
            }
            LanguageSearchType::Change => {
                helper.and_where_not_eq("status", "0");
            }
        };
        helper
    }
}