use crate::engine::Node;

pub trait NodeProvider {
    fn new_node() -> Box<dyn Node>;
}
