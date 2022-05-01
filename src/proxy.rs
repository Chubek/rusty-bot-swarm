use crate::config::Config;
use crate::utils::write_strings_to_zip;
use serde::Serialize;
use std::path::Path;
use thirtyfour::support::block_on;
use thirtyfour::{prelude::*, ChromeCapabilities};
use thirtyfour::{ExtensionCommand, RequestMethod};
use zip::write;

lazy_static! {
    static ref MANIFEST: String = String::from(
        r#"
    {
        "version": "1.0.0",
        "manifest_version": 3,
        "name": "Chrome Proxy",
        "permissions": [
        "Proxy",
        "Tabs",
        "unlimitedStorage",
        "Storage",
        "<all_urls>",
        "webRequest",
        "webRequestBlocking"
        ],
        "background": {
        "scripts": ["background.js"]
        },
        "Minimum_chrome_version":"76.0.0"
        }    
    "#
    );
    static ref USER_AGENT: String = String::from(
        "Mozilla/5.0 (Linux; Android 4.4.2; X325–Locked to Life Wireless Build/KOT49H) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/30.0.0.0 Mobile Safari/537.36 TwitterAndroid"
    );
}

pub struct Proxy {
    host: String,
    username: String,
    password: String,
}

impl Proxy {
    pub fn new(host: String, username: String, password: String) -> Self {
        Proxy {
            host,
            username,
            password,
        }
    }

    fn create_ext(&self) {
        let background = format!(
            r#"
        var config = {{
            mode: "fixed_servers",
            rules: {{
            singleProxy: {{
            scheme: "http",
            host: "{}",
            port: parseInt(PROXY_PORT)
            }},
            bypassList: ["foobar.com"]
            }}
            }};
            chrome.proxy.settings.set({{value: config, scope: "regular"}}, function() {{}});
            function callbackFn(details) {{
            return {{
            authCredentials: {{
            username: "{}",
            password: "{}"
            }}
            }};
            }}
            
            chrome.webRequest.onAuthRequired.addListener(
            callbackFn,
            {{urls: ["<all_urls>"]}},
            ['blocking']
            );

        
        "#,
            self.host, self.username, self.password
        );

        write_strings_to_zip(
            format!("./{}-{}.crx", self.host, self.username),
            background,
            MANIFEST.clone(),
        )
        .unwrap()
    }

    pub async fn launch_driver_with_proxy(&self, config: Config) -> WebDriverResult<WebDriver> {
        let proxy = Self::new(
            self.host.clone(),
            self.username.clone(),
            self.password.clone(),
        );

        let mut caps = ChromeCapabilities::new();

        caps.add_chrome_option("user-agent", USER_AGENT.clone())?;

        proxy.create_ext();
        let ext_str = format!("{}-{}.crx", proxy.host, proxy.username);
        let extension_path = Path::new(&ext_str);

        caps.add_extension(extension_path)?;

        let driver = WebDriver::new(&config.selenium_url, caps).await?;

        config.apply_config(&driver).await?;

        std::fs::remove_file(ext_str).unwrap();

        Ok(driver)
    }
}
