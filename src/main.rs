mod network;

#[tokio::main]
async fn main() {
    let local_addr = "localhost:8000";
    let remote_addr = "localhost:8001";

    // Start the network listener in a separate task.
    let listener_task = tokio::spawn(async move {
        network::start_network_listener(local_addr).await.unwrap();
    });

    // Start the network connector in a separate task.
    let connector_task = tokio::spawn(async move {
        network::start_network_connector(remote_addr).await.unwrap();
    });

    // Await both tasks.
    let _ = tokio::try_join!(listener_task, connector_task);
}