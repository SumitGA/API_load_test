#[derive(Parser, Debug, Clone)]
#[clap(name = "Rust HTTP Performance Test")]
#[clap(author = "Sumit Gautam")]
#[clap(version= "1.0")]
#[clap(about = "Rust CLI using clap libraru which sends HTTP request that can be use for performance test", long_about = None)]

struct Args {

    #[clap(value_parser, help = "HTTP method")]
    method: String,

    #[clap(value_parser, help = "Target URL")]
    url: String,

    #[clap(short, long, default_value_t = 1, help = "Number of Producers sending requests")]
    producers: u32,

    #[clap(short, long, default_value_t = 200, help = "Expected HTTP Return Status")]
    expected_status: u16,

    #[clap(short, long, default_value_t = 1000, help = "Number of Request to send")]
    requests: u32,

    #[clap(short, long, help = "Body for HTTP POST requests")]
    body: Option<String>,

    #[clap(short, long, default_value_t = 0, help = "Wait time in milliseconds between requests")]
    throttle_ms: u32,

    #[clap(short, long, default_value_t = -1, help = "Ramp up delay for each producer in milliseconds, this is the maximum time, a number between zero and this one will be selected. Default is the number of producers, set 0 to disable")]
    max_ramp_up_time: i32,
}

fn main() {
    println!("Hello, world!");
}
