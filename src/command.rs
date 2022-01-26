use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use run_script::ScriptOptions;
use crate::node_provider::NodeProvider;

pub trait Node {
    fn get_type(&self) -> String;
}

pub struct CurrentRunContext {
    pub node: Option<Box<dyn Node>>,
}

impl CurrentRunContext {
    pub fn get_node_type(&self) -> String {
        self.node.as_ref().map(|n| n.get_type()).unwrap_or("none".to_string())
    }
}

pub struct PoppyCommands {
    pub current_run_context: CurrentRunContext,
    node_provider: NodeProvider,
}

impl PoppyCommands {
    pub fn new() -> PoppyCommands {
        PoppyCommands {
            current_run_context: CurrentRunContext { node: None },
            node_provider: NodeProvider::new()
        }
    }

    pub fn node(&mut self, node: &str) -> Result<(), String> {
        let node = self.node_provider.get_node(node)?;
        self.current_run_context.node = Some(node);

        Ok(())
    }

    pub fn sh(&self, script: &str) -> Result<String, String> {
        if let Some(node) = &self.current_run_context.node {
            println!("running script: {} on node: {}", script, node.get_type());

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
            println!("running script file: {} on node: {}", script_file.to_str().unwrap(), node.get_type());

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
