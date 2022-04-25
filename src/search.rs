use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use thirtyfour::{error::WebDriverResult, WebDriver};

lazy_static! {
    static ref URL_ENC_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("+", "%2B");
        m.insert("\"", "%22");
        m.insert(":", "%3A");
        m.insert("#", "%23");
        m.insert("@", "%40");
        m
    };
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Search<'a> {
    all_words: Option<Vec<&'a str>>,
    exact_phrase: Option<Vec<&'a str>>,
    any_words: Option<Vec<&'a str>>,
    none_words: Option<Vec<&'a str>>,
    hashtags: Option<Vec<&'a str>>,
    language: Option<&'a str>,
    from_accounts: Option<Vec<&'a str>>,
    to_these_accounts: Option<Vec<&'a str>>,
    mentioning_accounts: Option<Vec<&'a str>>,
    minimum_replies: Option<u32>,
    minimum_likes: Option<u32>,
    minimum_retweets: Option<u32>,
    date_from: Option<&'a str>,
    date_to: Option<&'a str>,
}

impl<'a> Search<'a> {
    pub fn from_json_string(json_str: &'a str) -> Self {
        let search: Search = from_str(json_str).unwrap();

        search
    }

    pub fn format_text(self) -> String {
        let mut search_params = Vec::<String>::new();

        if let Some(all_word_vec) = self.clone().all_words {
            let all_word = all_word_vec.join("+");

            search_params.push(all_word);
        }

        if let Some(exact_phrases) = self.clone().exact_phrase {
            let exact_phrases_joined = exact_phrases.join("+");
            let exact_phrases_quoted = format!("\"{}\"", exact_phrases_joined);

            search_params.push(exact_phrases_quoted);
        }

        if let Some(any_words) = self.clone().any_words {
            let any_words_joined = any_words.join("+OR+");
            let any_words_para = format!("({})", any_words_joined);

            search_params.push(any_words_para);
        }

        if let Some(none_words) = self.clone().none_words {
            let none_words_joined = none_words.join("+-");
            let none_words_para = format!("-{}", none_words_joined);

            search_params.push(none_words_para);
        }

        if let Some(hashtags) = self.clone().none_words {
            let hashtags_words_joined = hashtags.join("+OR+");
            let hashtags_words_para = format!("({})", hashtags_words_joined);

            search_params.push(hashtags_words_para);
        }

        if let Some(from_accounts) = self.clone().from_accounts {
            let from_accounts_words_joined = from_accounts
                .iter()
                .map(|x| format!("from:{}", x))
                .collect::<Vec<String>>()
                .join("+OR+");

            let from_accounts_words_para = format!("({})", from_accounts_words_joined);

            search_params.push(from_accounts_words_para);
        }

        if let Some(to_accounts) = self.clone().from_accounts {
            let to_accounts_words_joined = to_accounts
                .iter()
                .map(|x| format!("to:{}", x))
                .collect::<Vec<String>>()
                .join("+OR+");

            let to_accounts_words_para = format!("({})", to_accounts_words_joined);

            search_params.push(to_accounts_words_para);
        }

        if let Some(mention_accounts) = self.clone().none_words {
            let mention_accounts_words_joined = mention_accounts.join("+OR+");
            let mention_accounts_words_para = format!("({})", mention_accounts_words_joined);

            search_params.push(mention_accounts_words_para);
        }

        if let Some(min_replies) = self.clone().minimum_replies {
            let min_rep_str = format!("min_replies:{}", min_replies);

            search_params.push(min_rep_str);
        }

        if let Some(min_favs) = self.clone().minimum_likes {
            let min_fav_str = format!("min_faves:{}", min_favs);

            search_params.push(min_fav_str);
        }

        if let Some(min_rt) = self.clone().minimum_retweets {
            let min_rt_str = format!("min_retweets:{}", min_rt);

            search_params.push(min_rt_str);
        }

        if let Some(lang) = self.clone().language {
            let lang_str = format!("lang::{}", lang);

            search_params.push(lang_str);
        }

        if let Some(until) = self.clone().date_to {
            let until_str = format!("since:{}", until);

            search_params.push(until_str);
        }

        if let Some(since) = self.clone().date_from {
            let since_str = format!("since:{}", since);

            search_params.push(since_str);
        }

        let mut params_joined = search_params.join("+");

        for (key, value) in URL_ENC_MAP.clone().into_iter() {
            params_joined = params_joined.replace(key, value);
        }

        params_joined = format!(
            "https://twitter.com/search?lang=en&q={}&src=typed_query",
            params_joined
        );

        params_joined
    }
}
