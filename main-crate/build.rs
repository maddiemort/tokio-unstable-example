#[tokio::main]
async fn main() {
    task_spawner::spawn_task("spawned_from_build_script").await;
}
