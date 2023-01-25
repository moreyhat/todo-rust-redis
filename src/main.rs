use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _: () = con.set("my_key", 42)?;
    con.get("my_key")
}

fn main() {
    let result = match fetch_an_integer() {
        Ok(value) => value,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("The result is {}", result);
}
