use rbatis_core::Result;
use crate::dao::RB;
use rbatis::crud::CRUD;
use crate::domain::domain::DBProject;
use crate::domain::vo::Project;

pub struct SysProjectService {}

impl SysProjectService {
    pub async fn all_project(&self) -> Result<Vec<Project>> {
        let r: Result<Vec<DBProject>> = RB.list("").await;
        match r {
            Ok(v) => {
                Ok(v.into_iter().map(|p| p.to_vo()).collect())
            }
            Err(e) => Err(e)
        }
    }
    pub async fn mergeUpdate(&self, project_id: i32) {}
}