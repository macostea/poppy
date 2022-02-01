node(NodeType::Docker, || {
    print(sh_file("tests/data/sh_file.sh"))
})
