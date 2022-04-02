use tokio::io::{AsyncWriteExt, AsyncReadExt, ReadHalf, split, WriteHalf, Result, Error};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // 客户端连接服务器
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    // 分离读写
    let (read, write) = split(stream);

    // 异步去写消息
    tokio::spawn(async move {
        // 调用写操作方法
        handle_write(write).await?;
        // 如果没有错误，则返回 Ok  这里用到tokio自带的Error。因为返回值要实现send+sync
        Ok::<_, Error>(())
    });

    // 读取服务端的消息
    handle_read(read).await
}

//处理写操作
async fn handle_write(mut write: WriteHalf<TcpStream>) -> Result<()> {
    // 创建一个空消息
    let mut message = String::new();
    // 模拟多次写入消息
    for _ in 1..5 {
        // 制作消息体
        message += "哈";
        // 写入消息
        write.write_all(message.as_bytes()).await?;
        // 等待3秒
        sleep(Duration::from_secs(3)).await;
    }

    // 写入消息结束
    Ok(())
}

//处理读操作
async fn handle_read(mut read: ReadHalf<TcpStream>) -> Result<()> {
    // 创建缓冲区
    let mut buf = [0; 1024];
    // 循环读取消息
    loop {
        // 读取消息
        let n = read.read(&mut buf).await?;
        // 如果没读取到就退出
        if n == 0 {
            break;
        }
        // 打印消息
        println!("Received server message：{}", String::from_utf8_lossy(&buf[..n]));
    }
    // 读取消息结束
    Ok(())
}