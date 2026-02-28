use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthLevel {
    None,
    Consumer,
    User,
}

#[derive(Debug, Clone)]
pub enum Auth {
    None,
    UserToken {
        token: String,
    },
    Discogs {
        consumer_key: String,
        consumer_secret: String,
    },
    OAuth {
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    },
}

impl Default for Auth {
    fn default() -> Self {
        Self::None
    }
}

impl Auth {
    pub fn level(&self) -> AuthLevel {
        match self {
            Auth::None => AuthLevel::None,
            Auth::Discogs { .. } => AuthLevel::Consumer,
            Auth::UserToken { .. } | Auth::OAuth { .. } => AuthLevel::User,
        }
    }

    pub fn authorization_header(&self) -> Option<String> {
        match self {
            Auth::None => None,
            Auth::UserToken { token } => Some(format!("Discogs token={token}")),
            Auth::Discogs {
                consumer_key,
                consumer_secret,
            } => Some(format!(
                "Discogs key={consumer_key}, secret={consumer_secret}"
            )),
            Auth::OAuth {
                consumer_key,
                consumer_secret,
                access_token,
                access_token_secret,
            } => Some(crate::oauth::build_oauth_header(
                consumer_key,
                consumer_secret,
                access_token,
                access_token_secret,
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    #[default]
    Discogs,
    Plaintext,
    Html,
}

impl OutputFormat {
    pub fn accept_header_value(self) -> &'static str {
        match self {
            OutputFormat::Discogs => "application/vnd.discogs.v2.discogs+json",
            OutputFormat::Plaintext => "application/vnd.discogs.v2.plaintext+json",
            OutputFormat::Html => "application/vnd.discogs.v2.html+json",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Auth, AuthLevel, OutputFormat};

    #[test]
    fn auth_levels_match_expected_capabilities() {
        assert_eq!(Auth::None.level(), AuthLevel::None);
        assert_eq!(
            Auth::Discogs {
                consumer_key: "k".into(),
                consumer_secret: "s".into(),
            }
            .level(),
            AuthLevel::Consumer
        );
        assert_eq!(
            Auth::UserToken { token: "t".into() }.level(),
            AuthLevel::User
        );
    }

    #[test]
    fn output_accept_header_is_stable() {
        assert_eq!(
            OutputFormat::Discogs.accept_header_value(),
            "application/vnd.discogs.v2.discogs+json"
        );
        assert_eq!(
            OutputFormat::Plaintext.accept_header_value(),
            "application/vnd.discogs.v2.plaintext+json"
        );
        assert_eq!(
            OutputFormat::Html.accept_header_value(),
            "application/vnd.discogs.v2.html+json"
        );
    }

    #[test]
    fn user_token_auth_header_format_is_stable() {
        let header = Auth::UserToken {
            token: "abc123".into(),
        }
        .authorization_header()
        .expect("header should exist");

        assert_eq!(header, "Discogs token=abc123");
    }
}
