use std::process::Command;

#[test]
/// Makes sure that the docker container can receive jobs and will process them
fn test_docker_compose() {
    let child = Command::new("./container-integration-test.sh")
        .output()
        .expect("Failed to run docker-compose up");

    let docker_compose_output: String = String::from_utf8(child.stdout).unwrap();
    println!("output: {}", docker_compose_output);
    assert!(docker_compose_output.contains(r#"args: [Object({"from": String("ruby"), "hello": String("world")})]"#));
}
