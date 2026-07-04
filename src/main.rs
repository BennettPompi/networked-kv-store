pub mod client;
pub mod server;
use arg_parse::{parse_args, Mode};
fn main() {
    let config = parse_args().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    match config.mode {
        Mode::Client => client::connect(config),
        Mode::Server => server::run(config),
    }
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    })
}
pub mod arg_parse {
    pub struct Config {
        pub mode: Mode,
        pub host: String,
        pub port: u16,
    }
    pub enum Mode {
        Client,
        Server,
    }
    enum Flag {
        Mode,
        Host,
        Port,
    }
    impl Default for Config {
        fn default() -> Self {
            Config {
                mode: Mode::Server,
                host: "localhost".to_owned(),
                port: 7242,
            }
        }
    }
    fn parse_flag(arg: &str) -> Result<Flag, String> {
        match arg {
            "-m" => Ok(Flag::Mode),
            "--host" => Ok(Flag::Host),
            "-p" => Ok(Flag::Port),
            _ => Err(format!("Flag not recognized: {arg}")),
        }
    }
    pub trait ConnectionString {
        fn get_connection_string(self) -> String;
    }
    impl ConnectionString for Config {
        fn get_connection_string(self) -> String {
            format!("{}:{}", self.host, self.port)
        }
    }
    pub fn parse_args() -> Result<Config, String> {
        let mut config = Config::default();
        let mut current_flag = None;
        for arg in std::env::args().skip(1) {
            match &current_flag {
                None => current_flag = Some(parse_flag(&arg)?),
                Some(flag) => {
                    match flag {
                        Flag::Mode => {
                            config.mode = match arg.as_str() {
                                "server" => Mode::Server,
                                "client" => Mode::Client,
                                _ => return Err(format!("Mode not recognized: {arg}")),
                            }
                        }
                        Flag::Host => config.host = arg,
                        Flag::Port => {
                            config.port =
                                arg.parse().map_err(|_| format!("Invalid Port: {arg}"))?;
                        }
                    };
                    current_flag = None
                }
            }
        }

        return Ok(config);
    }
}
