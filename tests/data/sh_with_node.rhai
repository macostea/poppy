node("docker", || {
    print(sh("date"))
})