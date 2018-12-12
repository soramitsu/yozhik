use std::env;
use std::fs::{read_to_string, File};

use rand::thread_rng;
use rand::seq::SliceRandom;
use lazy_static::lazy_static;


fn generate_random_token() -> String {
    static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789";
    String::from_utf8(ALPHABET.as_bytes().choose_multiple(&mut thread_rng(), 32).cloned().collect()).unwrap()
}

pub const GITHUB_API: &'static str = "https://api.github.com/";

const DIR_CONFIG_LOCAL: &'static str = "config/";
const DIR_CONFIG_SYSTEM: &'static str = "/etc/yozhik/";

const FILE_COMMENT: &'static str = "comment.md";
const FILE_WEBHOOK_KEY: &'static str = "webhook_key";

fn read_file(file: &str) -> std::io::Result<String> {
    read_to_string(format!("{}{}", DIR_CONFIG_LOCAL, file))
        .or_else(|_| read_to_string(format!("{}{}", DIR_CONFIG_SYSTEM, file)))
}

fn write_file(file: &str, content: &str) -> std::io::Result<()> {
    use std::io::prelude::*;
    let mut file = File::create(file)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Error messages
const ERROR_GITHUB_TOKEN: &'static str =
    "Environment property YOZHIK_GITHUB_TOKEN should contain your Github API token";
const ERROR_WEBHOOK_ADDRESS: &'static str =
    "Environment property YOZHIK_WEBHOOK_ADDRESS should contain your network bind address, e.g: 0.0.0.0:8080";
const ERROR_COMMENT: &'static str =
    "File /etc/yozhik/comment.md should contain the contents of issue closing message";
const ERROR_WEBHOOK_KEY: &'static str =
    "Could load or generate Github webhook key";


lazy_static! {
    pub static ref TOKEN: String = env::var("YOZHIK_GITHUB_TOKEN").expect(ERROR_GITHUB_TOKEN);
    pub static ref BIND_ADDRESS: String = env::var("YOZHIK_WEBHOOK_ADDRESS").expect(ERROR_WEBHOOK_ADDRESS);
    pub static ref COMMENT: String = read_file(FILE_COMMENT).expect(ERROR_COMMENT);
    pub static ref WEBHOOK_KEY: String =
        read_file(FILE_WEBHOOK_KEY).or_else(|_| {
            let new_token = generate_random_token();
            write_file(&format!("{}{}", DIR_CONFIG_SYSTEM, FILE_WEBHOOK_KEY), &new_token)
                .or_else(|_| write_file(&format!("{}{}", DIR_CONFIG_LOCAL, FILE_WEBHOOK_KEY), &new_token))
                .map(|_| new_token)
        }).expect(ERROR_WEBHOOK_KEY);
}
