use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct CurrentRunContext {
    pub node: Option<String>,
}

impl CurrentRunContext {
    pub fn get_node(&mut self) -> String {
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
        self.current_run_context.node = Some(node.to_string());
    }
}

pub type SharedPoppyCommands = Rc<RefCell<PoppyCommands>>;
