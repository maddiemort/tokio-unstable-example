pub async fn spawn_task(name: &str) {
    tokio::task::Builder::new()
        .name(name)
        .spawn(async {
            println!("running a task");
        })
        .unwrap()
        .await
        .unwrap();
}
