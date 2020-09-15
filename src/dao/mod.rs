// use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

lazy_static! {
    pub static ref RB:Rbatis={
      Rbatis::new()
    };
  }