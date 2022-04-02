use tokio::io::{AsyncReadExt, AsyncWriteExt};  // IO 异步读取和写入的 trait

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 监听客户端连接
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    // 循环接收客户端连接
    loop {
        // 接收客户端连接
        let (socket, socket_addr) = listener.accept().await?;
        // 打印客户端的地址
        println!("Accepted connection from: {}", socket_addr);
        // 异步处理客户端连接
        tokio::spawn(async move {
            // 调用方法
            handle_connection(socket).await;
        });
    }
}

// 处理客户端连接
async fn handle_connection(mut socket: tokio::net::TcpStream) {
    // 创建一个缓冲区
    let mut buf = [0; 1024];

    // 循环读取客户端发送的数据
    loop {
        // 用模式匹配 读取数据
        let n = match socket.read(&mut buf).await {
            // 如果是0字节就断开链接
            Ok(n) if n == 0 => return,
            // 大于0字节就读取
            Ok(n) => n,
            // 如果出错就打印错误信息并断开链接
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        // 字节数据转换成字符串
        let message = String::from_utf8_lossy(&buf[..n]);
        // 打印接收到的数据
        println!("Received client message: {}", message);

        // 定义一个响应数据
        let response = "呵".repeat(message.chars().count());

        // 发送响应数据 用模式匹配
        if let Err(e) = socket.write_all(response.as_bytes()).await {
            // 如果出错就打印错误信息并断开链接
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}