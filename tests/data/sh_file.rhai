node("docker", || {
    print(sh_file("tests/data/sh_file.sh"))
})
