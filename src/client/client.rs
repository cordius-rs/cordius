use reqwest::Client as Reqwest;
use dotenvy::from_path_iter;
use std::{fs, env, io::Write, path::Path};

pub struct Client {
    /// The token of the client
    pub token: String,
    /// Should the bot login from mobile?
    pub mobile: bool,
}

impl Client {

    /// Initializes a client
    /// # Arguments:
    /// token - accepts: &str - The token of the client
    ///
    /// # Returns
    /// Client
    pub async fn new(token: &str, debug: bool) -> Self {
        // Check if the token is valid
        if debug {
          Self::_is_valid_token(token).await;
        };

        // Check if .env exists and push/create .env file
        // so other structs can reach to client token
        // purely for design
        Self::_env_property_exists(token).await;

        Self {
            token: token.to_string(),
            mobile: false,
        }
    }

    /// Setter for the mobile property
    /// # Arguments
    /// device - accepts: bool - Should the bot login from mobile!
    ///
    /// # Returns
    /// Client 
    fn mobile(mut self, device: bool) -> Self {
        self.mobile = device;
        self
    }

    // Helper function
    // Checks if the given token is valid
    // If the token is invalid it panics
    async fn _is_valid_token(token: &str) {
        if token.is_empty() {
          panic!("Expected a valid token got an empty string!");
        } else {
            // Make a request to Discord API to see if the 
            // provided token is valid
            let req = Reqwest::new();
            let res = req
                .get("https://discord.com/api/v10/users/@me")
                .header("Authorization", format!("Bot {token}"))
                .send()
                .await
                .expect("Could not make a request to Discord API to validate the provided token, try again later.");

            // Provided token is invalid
            if res.status() != 200 {
                panic!("Expected a valid token got an invalid one");
            }
        }
    }

    // Helper Function
    // Checks if the "DISCORD_TOKEN" property exists on the .env file
    // If it does check if the token is valid via "_is_valid_token"
    // If it doesn't append the property
    async fn _env_property_exists(token: &str) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        if !path.exists() {
            fs::write(path, format!("DISCORD_TOKEN={token}")).expect("Could not create .env file!");
        } else {
            let exists = from_path_iter(&path)
                .unwrap()
                .any(|item| match item {
                    Ok((key, _)) => key == "DISCORD_TOKEN",
                    Err(_) => false,
                });

            if !exists {
                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .open(&path)
                    .expect("Could not open .env file!");

                writeln!(file, "DISCORD_TOKEN={token}").expect("Could not append content!");
            } else {
                let replaced: String = fs::read_to_string(&path).unwrap_or_default()
                    .lines()
                    .map(|line| {
                        if line.starts_with("DISCORD_TOKEN=") {
                            format!("DISCORD_TOKEN={token}")
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                fs::write(path, replaced).expect("Could not replace token in .env!");
            };
        };
    }
}
