node(NodeType::Docker, || {
    print("Hello world");
    debug(get_node_type());
});
