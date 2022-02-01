use crate::command::Node;

#[derive(Debug, Clone)]
pub struct DockerNode {

}

impl Node for DockerNode {
    fn get_type(&self) -> String {
        "docker".to_string()
    }
}

#[derive(Clone)]
pub enum NodeType {
    Docker,
}

pub struct NodeProvider {

}

impl NodeProvider {
    pub fn new() -> NodeProvider {
        NodeProvider {
        }
    }

    pub fn get_node(&self, node_type: NodeType) -> Result<Box<dyn Node>, String> {
        match node_type {
            NodeType::Docker => Ok(Box::new(DockerNode { })),
        }
    }
}
