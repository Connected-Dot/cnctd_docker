use std::env::set_current_dir;
use anyhow::anyhow;
use cnctd_shell::Shell;

pub struct DockerImage;

impl DockerImage {
    pub async fn publish(account: &str, image: &str, dockerfile_path: &str, project_dir: &str) -> anyhow::Result<()> {
        set_current_dir(project_dir)?;
    
        println!("building image");
    
        Shell::run(&format!("docker build --platform=linux/amd64 --tag {} --file {}/Dockerfile .", image, dockerfile_path), true).await?;
    
        Shell::run(&format!("docker tag {} {}/{}:latest", image, account, image), true).await?;
    
        Shell::run(&format!("docker push {}/{}:latest", account, image), true).await?;
    
        Shell::run(&format!("docker rmi {}", image), true).await?;
    
        Shell::run(&format!("docker image prune -f --filter dangling=true"), true).await?;
    
        Shell::run(&format!("docker system prune -f --filter until=60m --filter label=maintainer={}/{}", account, image), true).await?;
    
        Ok(())
    }
}

pub async fn check_for_docker() -> Result<(), anyhow::Error> {
    match Shell::run_with_exit_status("docker -v", false).await? {
        0 => Ok(()),
        _ => Err(anyhow!("Docker not installed"))
    }
}
