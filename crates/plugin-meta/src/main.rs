use log::{error, warn};
use reqwest::header::HeaderMap;
use std::{time::Duration, collections::HashMap};
use s3::{
    Bucket, Region,
    creds::Credentials,
    error::S3Error, BucketConfiguration, bucket_ops,
};
use once_cell::sync::OnceCell;

mod fabric;
mod forge;
// mod quilt;
mod minecraft;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DaedalusError(#[from] daedalus::Error),
    #[error("Error while deserializing JSON")]
    SerdeError(#[from] serde_json::Error),
    #[error("Unable to fetch {item}")]
    FetchError { inner: reqwest::Error, item: String },
    #[error("Error while managing asynchronous tasks")]
    TaskError(#[from] tokio::task::JoinError),
    #[error("Error while uploading file to S3")]
    S3Error {
        inner: S3Error,
        file: String,
    },
    #[error("Error while parsing version as semver: {0}")]
    SemVerError(#[from] semver::Error),
    #[error("Error while reading zip file: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("Error while reading zip file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error while obtaining strong reference to Arc")]
    ArcError,
}

static CLIENT: OnceCell<Bucket> = OnceCell::new();

#[tokio::main]
async fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("x-amz-acl", "public-read".parse().unwrap());
    let client = Bucket {
        name: dotenvy::var("S3_BUCKET_NAME").unwrap().into(),
        region: if &*dotenvy::var("S3_REGION").unwrap() == "r2" {
            Region::R2 {
                account_id: dotenvy::var("S3_URL").unwrap(),
            }
        } else {
            Region::Custom {
                region: dotenvy::var("S3_REGION").unwrap(),
                endpoint: dotenvy::var("S3_URL").unwrap(),
            }
        },
        credentials: std::sync::Arc::new(std::sync::RwLock::new(Credentials::new(
            Some(&*dotenvy::var("S3_ACCESS_TOKEN").unwrap()),
            Some(&*dotenvy::var("S3_SECRET").unwrap()),
            None,
            None,
            None,
        ).unwrap())),
        extra_headers: headers,
        extra_query: HashMap::new(),
        request_timeout: Some(Duration::from_secs(60)),
        path_style: false,
        listobjects_v2: true,
    };
    CLIENT.set(client).unwrap();

    env_logger::init();

    if check_env_vars() {
        error!("Some environment vars are missing!");
        return;
    }

    let mut timer = tokio::time::interval(Duration::from_secs(10 * 60));

    loop {
        timer.tick().await;

        let mut uploaded_files = Vec::new();

        let versions = match minecraft::retrieve_data(&mut uploaded_files).await {
            Ok(res) => Some(res),
            Err(err) => {
                error!("{:?}", err);

                None
            }
        };

        if let Some(manifest) = versions {
            match fabric::retrieve_data(&manifest, &mut uploaded_files).await {
                Ok(..) => {}
                Err(err) => error!("{:?}", err),
            };
            match forge::retrieve_data(&manifest, &mut uploaded_files).await {
                Ok(..) => {}
                Err(err) => error!("{:?}", err),
            };
            /* 
            match quilt::retrieve_data(&manifest, &mut uploaded_files).await {
                Ok(..) => {}
                Err(err) => error!("{:?}", err),
            };
            */
        }
    }
}

fn check_env_vars() -> bool {
    let mut failed = false;

    fn check_var<T: std::str::FromStr>(var: &str) -> bool {
        if dotenvy::var(var)
            .ok()
            .and_then(|s| s.parse::<T>().ok())
            .is_none()
        {
            warn!(
                "Variable `{}` missing in dotenvy or not of type `{}`",
                var,
                std::any::type_name::<T>()
            );
            true
        } else {
            false
        }
    }

    failed |= check_var::<String>("BASE_URL");
    failed |= check_var::<String>("BASE_FOLDER");

    failed |= check_var::<String>("S3_ACCESS_TOKEN");
    failed |= check_var::<String>("S3_SECRET");
    failed |= check_var::<String>("S3_URL");
    failed |= check_var::<String>("S3_REGION");
    failed |= check_var::<String>("S3_BUCKET_NAME");

    failed
}

pub async fn upload_file_to_bucket(
    path: String,
    bytes: Vec<u8>,
    content_type: Option<String>,
    uploaded_files: &tokio::sync::Mutex<Vec<String>>,
) -> Result<(), Error> {
    let key = format!("{}/{}", &*dotenvy::var("BASE_FOLDER").unwrap(), path);

    for attempt in 1..=4 {
        let result = if let Some(ref content_type) = content_type {
            CLIENT.get().unwrap().put_object_with_content_type(
                key.clone(),
                &bytes,
                content_type,
            ).await
        } else {
            CLIENT.get().unwrap().put_object(
                key.clone(),
                &bytes,
            ).await
        }.map_err(|err| Error::S3Error {
            inner: err,
            file: format!("{}/{}", &*dotenvy::var("BASE_FOLDER").unwrap(), path),
        });

        match result {
            Ok(_) => {
                {
                    let mut uploaded_files = uploaded_files.lock().await;
                    uploaded_files.push(key);
                }

                return Ok(());
            }
            Err(_) if attempt <= 3 => continue,
            Err(_) => {
                result?;
            }
        }
    }

    unreachable!()
}

pub fn format_url(path: &str) -> String {
    format!(
        "{}/{}/{}",
        &*dotenvy::var("BASE_URL").unwrap(),
        &*dotenvy::var("BASE_FOLDER").unwrap(),
        path
    )
}