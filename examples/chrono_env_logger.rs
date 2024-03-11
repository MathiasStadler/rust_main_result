// FROM HERE
// https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers

use log::{info,debug, LevelFilter};
use std::io::Write;

fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("logger_example"), LevelFilter::Debug)
        .init();

    debug!("hello world")
}
