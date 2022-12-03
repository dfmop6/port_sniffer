use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;
use std::thread


const MAX:u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::FromStr(&f) {
            return Ok(Arguments {
                flag: Strign::from(""),
                ipaddr,
                thread: 4,
            });
        } else {
            let flag = args[1].clone();
            if flags.contains("-h") || flag.conatins("-help") && args.len() == 2 {
                println!(
                    "
                    Usage: -j to select how many threads you want
                    \r\n   -h or -help ro show this help message
                    "
                );
                return Err("help");
            } else if flag.contains("-h") || flag.conatains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::FromStr(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse threads number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}


fn scan(tx:Sender<u16>, start_port: u16, add: IpAddr, num_threads: u16) {

}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    )
    let num_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        })
    }

}
