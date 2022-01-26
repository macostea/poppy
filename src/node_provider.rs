use crate::command::Node;

#[derive(Debug, Clone)]
pub struct DockerNode {

}

impl Node for DockerNode {
    fn get_type(&self) -> String {
        "docker".to_string()
    }
}

pub struct NodeProvider {

}

impl NodeProvider {
    pub fn new() -> NodeProvider {
        NodeProvider {
        }
    }

    pub fn get_node(&self, node_type: &str) -> Result<Box<dyn Node>, String> {
        match node_type {
            "docker" => Ok(Box::new(DockerNode { })),
            _ => Err("Unknown node type".to_string()),
        }
    }
}
