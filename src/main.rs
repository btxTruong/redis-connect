use redis_connect::RedisCli;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");
        let input = input.trim();

        if input == "break" {
            break;
        }

        let mut redis = RedisCli::connect("127.0.0.1", 6379);
        redis.send(format!("{}\n", input).as_bytes());
        let byte_receive = redis.receive();
        println!("{}", byte_receive);
    }
}