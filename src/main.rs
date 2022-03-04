use std::process::Command;
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path!().map(|| {
        Command::new("git").arg("-C").arg("/home/$USER/balasolu").arg("pull").spawn().expect("unable to git pull");
        Command::new("sudo").arg("docker-compose").arg("-f").arg("/home/$USER/balasolu/docker-compose-release.yml").arg("down").spawn().expect("unable to run docker command");
        Command::new("sudo").arg("docker-compose").arg("-f").arg("/home/$USER/balasolu/docker-compose-release.yml").arg("up").arg("-d").arg("--build").spawn().expect("unable to run docker command");
        format!("{}", "")
    });

    warp::serve(index)
        .run(([0, 0, 0, 0], 5321))
        .await;
}