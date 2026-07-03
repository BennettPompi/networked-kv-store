pub mod server;
enum Mode {
    Client,
    Server,
}
enum Flag {
    Mode,
    Host,
    Port,
}
struct Config {
    mode: Mode,
    host: String,
    port: u16,
}
fn default_config() -> Config {
    Config {
        mode: Mode::Server,
        host: "localhost".to_owned(),
        port: 7242,
    }
}
fn main() {
    let config = parse_args();
    match config.mode {
        Mode::Client => panic!("Not Implemented!"),
        Mode::Server => server::listen(),
    }
}
fn parse_flag(arg: &str) -> Flag {
    match arg {
        "-m" => Flag::Mode,
        "--host" => Flag::Host,
        "-p" => Flag::Port,
        _ => panic!("Flag not recognized: {arg}"),
    }
}
fn parse_args() -> Config {
    let mut config: Config = default_config();
    let mut current_flag: Option<Flag> = None;
    for arg in std::env::args() {
        match &current_flag {
            None => current_flag = Some(parse_flag(&arg)),
            Some(flag) => {
                match flag {
                    Flag::Mode => {
                        config.mode = match arg.as_str() {
                            "server" => Mode::Server,
                            "client" => Mode::Client,
                            _ => panic!("Mode not recognized: {arg}"),
                        }
                    }
                    Flag::Host => config.host = arg,
                    Flag::Port => {
                        config.port = arg.parse().unwrap();
                    }
                };
                current_flag = None
            }
        }
    }

    return config;
}
