#[allow(unused)]
use std::{path::PathBuf, time::Duration};
use url::Url;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub public_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub rate_limit: u8,
    pub rate_limit_expires: Duration,
    #[cfg(feature = "tls")]
    pub cert_file: PathBuf,
    #[cfg(feature = "tls")]
    pub key_file: PathBuf,
}

macro_rules! config_option {
    (env $env:literal $(=> $transform:expr)?$(, default $default:expr)?) => {
        std::env::var(String::from($env))
            .ok()
            $(.and_then($transform))?
            $(.unwrap_or_else(|| $default))?
    }
}

impl Config {
    pub fn init() -> eyre::Result<Self> {
        let port = config_option!(
            env "POLYUI_PORT" => |it| it.parse::<u16>().ok(),
            default 8080
        );

        let host = config_option!(
            env "POLYUI_HOST",
            default String::from("127.0.0.1")
        );

        let public_url = config_option!(
            env "POLYUI_PUBLIC_URL" => |it| Url::parse(&it).ok(),
            default Url::parse("https://127.0.0.1:8080/").unwrap()
        );

        let client_id = config_option!(
            env "POLYUI_CLIENT_ID",
            default String::from("9eac3a4e-8cdd-43ef-863e-49cd601b1f03")
        );
        // .ok_or_else(|| eyre::eyre!("Could not find PolyUI client ID"))?;

        let client_secret = config_option!(
            env "POLYUI_CLIENT_SECRET",
            default String::from("AGl8Q~RWp0A~WSNB.dN-9n.kNNQk7So0KlN7lbtb")
        );
        // .ok_or_else(|| eyre::eyre!("Could not find PolyUI client secret"))?;

        let rate_limit = config_option!(
            env "POLYUI_RATE_LIMIT" => |it| it.parse::<u8>().ok(),
            default 10
        );

        let rate_limit_expires = config_option!(
            env "POLYUI_RATE_LIMIT_EXPIRES" => |it| {
                let minutes = it.parse::<u64>().ok()?;
                Some(Duration::from_secs(minutes * 60))
            },
            default Duration::from_secs(30 * 60)
        );

        #[allow(unused)]
        let dirs = directories::ProjectDirs::from("cc", "Polyfrost", "PolyUI");

        #[cfg(feature = "tls")]
        let (cert_file, key_file) = (
            config_option!(
                env "POLYUI_CERT" => |it| Some(PathBuf::from(it))
            )
            .or_else(|| Some(dirs.as_ref()?.config_dir().join("cert.pem")))
            .ok_or(eyre::eyre!(
                "Could not find SSL certificate in any location"
            ))?,
            config_option!(
                env "POLYUI_KEY" => |it| Some(PathBuf::from(it))
            )
            .or_else(|| Some(dirs.as_ref()?.config_dir().join("key.pem")))
            .ok_or(eyre::eyre!("Could not find SSL key in any location"))?,
        );

        Ok(Self {
            port,
            host,
            public_url,
            client_id,
            client_secret,
            rate_limit,
            rate_limit_expires,
            #[cfg(feature = "tls")]
            cert_file,
            #[cfg(feature = "tls")]
            key_file,
        })
    }
}