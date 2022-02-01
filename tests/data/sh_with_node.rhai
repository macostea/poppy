node(NodeType::Docker, || {
    print(sh("date"))
})