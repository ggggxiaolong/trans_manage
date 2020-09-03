// use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

lazy_static! {
    pub static ref RB:Rbatis={
      Rbatis::new()
      //  let mut rb = Rbatis::new();
      //  let del = RbatisLogicDeletePlugin::new("del");
      //  rb.logic_plugin = Some(Box::new(del));
      //  rb
    };
  }