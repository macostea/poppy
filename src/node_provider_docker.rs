use crate::{node_provider::NodeProvider, engine::Node};

#[derive(Debug, Clone)]
pub struct DockerNode {

}

impl Node for DockerNode {
    fn name(&self) -> String {
        todo!()
    }
}
pub struct DockerNodeProvider {

}

impl NodeProvider for DockerNodeProvider {
    fn new_node() -> Box<dyn Node> {
        Box::new(DockerNode { })
    }
}