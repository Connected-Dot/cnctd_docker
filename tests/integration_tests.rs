#[cfg(test)]
mod tests {
    use cnctd_docker::check_for_docker;

    #[tokio::test]
    async fn test_commands() {
        check_for_docker().await.unwrap();
    }
}
