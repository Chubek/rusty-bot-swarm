use chrono::{DateTime, NaiveDateTime, Utc};
use rand::{self, Rng};
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use zip::write::FileOptions;


lazy_static! {
    pub static ref REMAINDER_STR: String = String::from(r#"
        includePromotedContent%22%3Atrue%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Atrue%2C%22__fs_responsive_web_like_by_author_enabled%22%3Afalse%2C%22__fs_dont_mention_me_view_api_enabled%22%3Atrue%2C%22__fs_interactive_text_enabled%22%3Atrue%2C%22__fs_responsive_web_uc_gql_enabled%22%3Afalse%2C%22__fs_responsive_web_edit_tweet_api_enabled%22%3Afalse%7D
    "#);
}

pub fn write_to_file(fname: &str, message: String) -> std::io::Result<()> {
    let mut file = File::create(fname)?;
    file.write_all(message.as_bytes())?;
    Ok(())
}

pub fn read_from_file(fname: &str) -> std::io::Result<String> {
    let contents = read_to_string(fname)?;

    Ok(contents)
}

pub fn convert_timestamp(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);

    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime
}

pub fn rand_num_wait() -> u8 {
    rand::thread_rng().gen_range(120..255)
}

pub fn write_strings_to_zip(
    filename: String,
    background: String,
    manifest: String,
) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename.as_str());
    let file = std::fs::File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip.start_file(format!("background.js"), options)?;
    zip.write_all(background.as_bytes())?;

    zip.start_file(format!("manifest.json"), options)?;
    zip.write_all(manifest.as_bytes())?;

    Ok(())
}

pub fn today_date_coll_name() -> String {
    let now = Utc::now();

    let mut ret = now.date().format("%Y-%m-%d").to_string();

    ret.push_str("-posts");

    ret
}


pub fn make_get_post_url(id: String, count: u32, linkid: String) -> String {
    let domain_id = format!("https://twitter.com/i/api/graphql/{}/UserTweets?variables=", linkid);
    let params_main = format!("%7B%22userId%22%3A%{}%22%2C%22count%22%3A{}%2C%22", id, count);

    let fin = format!("{}{}{}", domain_id, params_main, REMAINDER_STR.clone()).to_string();

    fin
}