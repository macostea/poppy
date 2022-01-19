use core::time;
use std::thread;

use rhai::{EvalAltResult, Engine, ImmutableString, Dynamic, NativeCallContext, FnPtr, Scope};


fn add_len(x: i64, s: ImmutableString) -> i64 {
    x + s.len() as i64
}

fn add_len_str(x: i64, s: &str) -> i64 {
    x + s.len() as i64
}

fn get_any_value() -> Dynamic {
    42_i64.into()
}

fn safe_divide(x: i64, y:i64) -> Result<i64, Box<EvalAltResult>> {
    if y == 0 {
        Err("Division by zero!".into())
    } else {
        Ok(x / y)
    }
}

fn greet(context: NativeCallContext, callback: FnPtr) -> Result<ImmutableString, Box<EvalAltResult>> {
    let name: ImmutableString = callback.call_within_context(&context, ())?;

    Ok(format!("hello, {}!", name).into())
}

fn wait(seconds: i64) {
    thread::sleep(time::Duration::from_secs(seconds as u64));
}

#[derive(Debug, Clone)]
struct TestStruct {
    x: i64,
    y: i64,
}

impl TestStruct {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    
    fn update(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }
}

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine
        .register_fn("add", add_len)
        .register_fn("add_str", add_len_str)
        .register_fn("get_any_value", get_any_value)
        .register_result_fn("divide", safe_divide)
        .register_result_fn("greet", greet)
        .register_fn("wait", wait);

    engine
        .register_type::<TestStruct>()
        .register_fn("new_ts", TestStruct::new)
        .register_fn("update_ts", TestStruct::update);
    
    let result = engine.eval::<i64>(r#"add(40, "xx")"#)?;
    println!("Answer: {}", result);

    if let Err(error) = engine.eval::<i64>("divide(40, 0)") {
        println!("Error: {:?}", *error);
    }

    let mut greet_script_ast = engine.compile(r#"
        fn get_name() {
            print("Starting to run callback");
            // wait(3);
            print("End callback run");
            return "Mihai";
        }

        return greet(Fn("get_name"))
    "#)?;

    let greet_result = engine.eval_ast::<ImmutableString>(&greet_script_ast)?;

    println!("Greet result: {}", greet_result);

    let mut scope = Scope::new();

    greet_script_ast.clear_statements();
    let name: ImmutableString = engine.call_fn(&mut scope, &greet_script_ast, "get_name", ())?;

    println!("Result from fn call: {}", name);

    let result = engine.eval::<TestStruct>(
        "
            let ts = new_ts(1, 2);
            update_ts(ts, 3, 4);
            return ts;
        "
    )?;

    println!("result: {:?}", result);

    Ok(())
}
