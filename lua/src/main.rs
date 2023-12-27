use core::time::Duration;
use mlua::prelude::*;
use tokio::time;

async fn sleep(_: &Lua, ms: u64) -> LuaResult<()> {
    time::sleep(Duration::from_millis(ms)).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let lua = unsafe { Lua::unsafe_new() };
            let lua_global = lua.globals();
            lua_global
                .set("sleep", lua.create_async_function(sleep).unwrap())
                .unwrap();
            drop(lua_global);

            lua.load(
                r###"
                hello = require("module")
                tom = hello.Human.new("Tom")
                print(tom:hello())
                "###,
            )
            .exec_async()
            .await
            .unwrap();
            lua.load("while true do sleep(1) end")
                .exec_async()
                .await
                .unwrap();
        })
    });

    loop {
        let memory_info = psutil::process::Process::current()
            .unwrap()
            .memory_info()
            .unwrap();
        println!("memory_info: {:?}", memory_info);
        time::sleep(Duration::from_millis(10_000)).await;
    }
}
