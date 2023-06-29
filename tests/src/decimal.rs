use tarantool::{decimal, decimal::Decimal, tlua, tuple::Tuple};

pub fn from_lua() {
    let d: Decimal = tarantool::lua_state()
        .eval("return require('decimal').new('-8.11')")
        .unwrap();
    assert_eq!(d.to_string(), "-8.11");
}

pub fn to_lua() {
    let lua = tarantool::lua_state();
    let tostring: tlua::LuaFunction<_> = lua.eval("return tostring").unwrap();
    let d: Decimal = "-8.11".parse().unwrap();
    let s: String = tostring.call_with_args(d).unwrap();
    assert_eq!(s, "-8.11");
}

pub fn from_tuple() {
    let t: Tuple = tarantool::lua_state()
        .eval("return box.tuple.new(require('decimal').new('-8.11'))")
        .unwrap();
    let (d,): (Decimal,) = t.decode().unwrap();
    assert_eq!(d.to_string(), "-8.11");
}

pub fn to_tuple() {
    let d = decimal!(-8.11);
    let t = Tuple::new(&[d]).unwrap();
    let lua = tarantool::lua_state();
    let f: tlua::LuaFunction<_> = lua.eval("return box.tuple.unpack").unwrap();
    let d: Decimal = f.call_with_args(&t).unwrap();
    assert_eq!(d.to_string(), "-8.11");
}
