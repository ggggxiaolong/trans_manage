mod sys_lang_service;
mod sys_user_service;
mod sys_project_service;

use sys_lang_service::SysLangService;
use sys_user_service::SysUserService;
use sys_project_service::SysProjectService;

lazy_static! {
    pub static ref SYS_USER_SERVICE: SysUserService = SysUserService{};
    pub static ref SYS_LANG_SERVICE: SysLangService = SysLangService{};
    pub static ref SYS_PROJECT_SERVICE: SysProjectService = SysProjectService{};
}