use std::process::Command;
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path!().map(|| {
        Command::new("cd").arg("~/balasolu").spawn().expect("could not change directories");
        Command::new("git").arg("pull").spawn().expect("unable to git pull");
        Command::new("sudo").arg("docker-compose").arg("-f").arg("docker-compose-release.yml").arg("down").spawn().expect("unable to run docker command");
        Command::new("sudo").arg("docker-compose").arg("-f").arg("docker-compose-release.yml").arg("up").arg("-d").arg("--build").spawn().expect("unable to run docker command");
        format!("{}", "")
    });

    warp::serve(index)
        .run(([127, 0, 0, 1], 5321))
        .await;
}