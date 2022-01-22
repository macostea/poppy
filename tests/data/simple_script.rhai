node("test_node", || {
    print("Hello world");
    debug(current_context().node);
});
