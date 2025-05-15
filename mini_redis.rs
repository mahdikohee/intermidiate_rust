use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // কনকট Redis সরভর
    let mut conn = client::connect("127.0.0.1:6379").await?;

    // ভযল সট কর
    conn.set("Hello", "Sohee al mahdi".into()).await?;

    // ভযল পড় আন
    let result = conn.get("Hello").await?;

    // ফলফল দখও
    println!("Got value from hello is: {:?}", result);

    Ok(())
}
