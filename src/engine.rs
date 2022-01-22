use std::path::PathBuf;
use std::cell::RefCell;
use rhai::{Engine, packages::{CorePackage, Package}, AST, NativeCallContext, FnPtr, EvalAltResult};

use crate::command::{PoppyCommands, SharedPoppyCommands, CurrentRunContext};

pub struct PoppyEngine {
    rhai_engine: Engine,
    script_ast: Option<AST>,
}

impl PoppyEngine {
    pub fn new() -> PoppyEngine {
        let engine = PoppyEngine::init_rhai_engine();
        PoppyEngine { rhai_engine: engine, script_ast: None }
    }

    pub fn load_script(&mut self, script_path: PathBuf) -> Result<(), String> {
        let ast = self.rhai_engine.compile_file(script_path).map_err(|e| {
            e.to_string()
        })?;

        self.script_ast = Some(ast);

        Ok(())
    }

    pub fn run_script(&mut self) -> Result<(), String> {
        if let Some(ast) = self.script_ast.take() {
            Ok(self.rhai_engine.run_ast(&ast).map_err(|e| {
                e.to_string()
            })?)
        } else {
            Err("No script loaded".to_string())
        }
    }

    fn init_rhai_engine() -> Engine {
        let mut engine = Engine::new_raw();
        let package = CorePackage::new();
        engine.register_global_module(package.as_shared_module());

        engine.on_print(|x| println!("Script: {}", x));
        engine.on_debug(|x, src, pos| {
            let src = src.unwrap_or("<unknown>");
            println!("Script: {}:{} {}", src, pos, x)
        });

        engine
            .register_type::<CurrentRunContext>()
            .register_get("node", CurrentRunContext::get_node);

        let commands = SharedPoppyCommands::new(RefCell::new(PoppyCommands::new()));

        let c = commands.clone();
        engine.register_result_fn("node", move |context: NativeCallContext, node: &str, function: FnPtr| -> Result<(), Box<EvalAltResult>> {
            c.borrow_mut().node(node);

            function.call_within_context(&context, ())?;
            Ok(())
        });

        let c = commands.clone();
        engine.register_fn("current_context", move || {
            c.borrow().current_run_context.clone()
        });

        engine
    }
}
