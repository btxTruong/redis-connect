use redis_connect::RedisCli;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Ok(mut redis) = RedisCli::connect("127.0.0.1:6379") {

            redis.send(format!("{}\n", input).as_bytes());

            let byte_receive = redis.receive();
            println!("{}", byte_receive);
        } else {
            println!("Can't not connect Redis server. Try again ...");
            continue;
        }
    }
}