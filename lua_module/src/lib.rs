#![allow(unused_parens)]
#![allow(unused_variables)]

#[derive(Clone)]
struct Human {
    name: String,
}
impl Human {
    fn new(name: String) -> mlua::Result<Human> {
        Ok(Human { name })
    }

    fn hello(&self) -> mlua::Result<String> {
        Ok(format!("hello, {}", self.name))
    }
}
impl mlua::UserData for Human {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |lua, (name): (String)| Self::new(name));
        methods.add_method("hello", |lua, this, (): ()| this.hello());
    }
}

#[mlua::lua_module(name = "module")]
fn module(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let m = lua.create_table()?;
    m.set("Human", lua.create_proxy::<Human>()?)?;
    Ok(m)
}
