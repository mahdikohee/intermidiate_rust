use mini_redis::{client , Result};
#[tokio::main]
pub async fn main() -> Result<()> {

    let mut connection = client::connect("192.168.104:333").await?;
    connection.set("hello" , "Sohee AL MAhdi Dibbo".into()).await?;
    let result = connection.get("hello").await?;
    if let Some(v) = result {

        println!("The value is :{:?}" , v);
    }else {

        eprintln!("No value found !");
    }
    Ok(())
} 
 
