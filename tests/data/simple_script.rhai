node("docker", || {
    print("Hello world");
    debug(get_node_type());
});
