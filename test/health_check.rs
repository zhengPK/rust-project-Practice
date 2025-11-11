#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
// 端口0在操作系统级别是一个特例：尝试绑定端口0将触发操作系统扫描可用端口，然后将其绑定到应用程序
fn spawn_app() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // 获取操作系统分配的端口
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)?.expect("Failed to run server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port);
}