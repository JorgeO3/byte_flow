pub mod bencode;
pub mod prelude;
pub mod torrent;

pub mod core {
    use clap::Parser;

    pub use crate::bencode;
    pub use crate::prelude::*;
    pub use crate::torrent;

    const VERSION: &str = "1.0.0";
    const AUTHOR: &str = "Jorge Osorio";
    const ABOUT: &str = "A BitTorrent client written in Rust";

    #[derive(Debug, Parser)]
    #[clap(version = VERSION, author = AUTHOR, about = ABOUT)]
    struct Args {
        #[clap(short, long)]
        torrent_file: String,
        #[clap(short, long)]
        output_dir: String,
        #[clap(short = 'n', long, default_value_t = 5)]
        thread_num: usize,
        #[clap(short, long, default_value_t = false)]
        logging: bool,
        #[clap(short = 'f', long, default_value = "../logs/client.log")]
        log_file: String,
    }

    pub fn run() -> Result<()> {
        let Args {
            torrent_file,
            output_dir,
            thread_num,
            logging,
            log_file,
        } = Args::parse();

        if logging {
            enable_loggin(&log_file)
        };

        log::info!("Starting client");
        OK
    }

    fn enable_loggin(file: &str) {
        use log::LevelFilter;
        use log4rs::append::file::FileAppender;
        use log4rs::config::{Appender, Config, Root};
        use log4rs::encode::pattern::PatternEncoder;

        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
            .build(file)
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
}
