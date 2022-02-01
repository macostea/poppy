use std::{path::PathBuf};
use std::cell::RefCell;
use rhai::{Engine, packages::{CorePackage, Package}, AST, NativeCallContext, FnPtr, EvalAltResult};
use rhai::plugin::*;

use crate::command::{PoppyCommands, SharedPoppyCommands};
use crate::node_provider::NodeType;

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
            match self.rhai_engine.run_ast(&ast) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string())
            }
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

        let commands = SharedPoppyCommands::new(RefCell::new(PoppyCommands::new()));

        engine.register_type_with_name::<NodeType>("NodeType")
            .register_static_module("NodeType", exported_module!(node_type_module).into());

        let c = commands.clone();
        engine.register_result_fn("node", move |context: NativeCallContext, node: NodeType, function: FnPtr| -> Result<(), Box<EvalAltResult>> {
            c.borrow_mut().node(node)?;

            function.call_within_context(&context, ())?;
            Ok(())
        });

        let c = commands.clone();
        engine.register_fn("get_node_type", move || {
            c.borrow_mut().current_run_context.get_node_type()
        });

        let c = commands.clone();
        engine.register_result_fn("sh", move |script: &str| -> Result<String, Box<EvalAltResult>> {
            c.borrow().sh(script).map_err(|e| e.into())
        });

        let c = commands.clone();
        engine.register_result_fn("sh_file", move |script_file: &str| -> Result<String, Box<EvalAltResult>> {
            c.borrow().sh_file(script_file.into()).map_err(|e| e.into())
        });

        engine
    }
}

#[export_module]
#[allow(non_upper_case_globals)]
mod node_type_module {
    pub const Docker: NodeType = NodeType::Docker;
}
