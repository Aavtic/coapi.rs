use std::process::Command;

static DOCKER_NAME: &str = "secure-python";

pub fn docker_build() {
    let _ = Command::new("sudo")
        .args(["docker", "build", "-t", DOCKER_NAME, "."])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

}

