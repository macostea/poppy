use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use run_script::ScriptOptions;

use crate::engine::Node;

#[derive(Debug, Clone)]
pub struct CurrentRunContext {
    pub node: Option<Box<dyn Node>>,
}

impl CurrentRunContext {
    pub fn get_node(&mut self) -> Box<dyn Node> {
        self.node.clone().unwrap_or("none".to_string())
    }
}

pub struct PoppyCommands {
    pub current_run_context: CurrentRunContext,
}

impl PoppyCommands {
    pub fn new() -> PoppyCommands {
        PoppyCommands { current_run_context: CurrentRunContext { node: None } }
    }

    pub fn node(&mut self, node: &str) {
        println!("node: {}", node);
        // TODO: Create or connect to node to run
        self.current_run_context.node = Some(node.to_string());
    }

    pub fn sh(&self, script: &str) -> Result<String, String> {
        if let Some(node) = &self.current_run_context.node {
            println!("running script: {} on node: {}", script, node);

            let options = ScriptOptions::new();
            let args = vec![];

            let (code, output, error) = run_script::run(script, &args, &options,).unwrap();
            match code == 0 {
                true => return Ok(output),
                false => return Err(error)
            };
        }

        Err("Cannot run sh outside a node".to_string())
    }

    pub fn sh_file(&self, script_file: PathBuf) -> Result<String, String> {
        if let Some(node) = &self.current_run_context.node {
            println!("running script file: {} on node: {}", script_file.to_str().unwrap(), node);

            let options = ScriptOptions::new();
            let args = vec![];

            let script = format!("sh {}", script_file.to_str().unwrap());

            let (code, output, error) = run_script::run(script.as_str(), &args, &options,).unwrap();
            match code == 0 {
                true => return Ok(output),
                false => return Err(error)
            };
        }

        Err("Cannot run sh_file outside a node".to_string())
    }
}

pub type SharedPoppyCommands = Rc<RefCell<PoppyCommands>>;
