use std::process::Command;
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path!().map(|| {
        std::env::set_current_dir("/home/$USER/balasolu").expect("could not change directories");
        Command::new("git").arg("pull").spawn().expect("unable to git pull");
        Command::new("sudo").arg("docker-compose").arg("-f").arg("docker-compose-release.yml").arg("down").spawn().expect("unable to run docker command");
        Command::new("sudo").arg("docker-compose").arg("-f").arg("docker-compose-release.yml").arg("up").arg("-d").arg("--build").spawn().expect("unable to run docker command");
        format!("{}", "")
    });

    warp::serve(index)
        .run(([0, 0, 0, 0], 5321))
        .await;
}