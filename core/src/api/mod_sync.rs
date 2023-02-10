extern crate reqwest;

use futures::StreamExt;

use super::mod_extraction::extract_info_from_jar;
use crate::entities::model::mod_type::JARLoadedMod;
use crate::entities::model::modrinth_mod_metadata::ModrinthModMetadata;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Retrieve MR mod meta-data from https://docs.modrinth.com/api-spec/#tag/version-files/operation/versionFromHash.
/// TODO: CurseForge support.
#[tracing::instrument]
async fn retrieve_mod_metadata(file_hash: &String, hashing_alg: &String) -> crate::error::Result<ModrinthModMetadata> {
    let url = format!("https://api.modrinth.com/v2/version_file/{}", 
        file_hash);
    let params = [("algorithm", hashing_alg)];

    let url = reqwest::Url::parse_with_params(&url, &params)?;

    Ok(crate::config::REQWEST_CLIENT
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<ModrinthModMetadata>()
        .await?)
}


fn list_jars(dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let valid_extensions = vec![String::from("jar"), String::from("zip")];

    let mut jars: Vec<String> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                jars.extend(list_jars(&path)?);
            } else if valid_extensions.contains(&path.extension().and_then(OsStr::to_str)
                .unwrap_or_else(|| "").to_string()) {
                let path_str = path.to_str().expect("Could not retrieve path.");
                jars.push(path_str.to_string());
            }
        }
    }
    Ok(jars)
}

// Get mods installed for a profile given the directory. 
async fn get_local_mods(dir: &Path) -> crate::error::Result<Vec<JARLoadedMod>> {
    let jars = list_jars(dir).ok().expect("Could not get JARs");

    let stream = futures::stream::iter(jars);
    let mods = stream
        .then(|path| extract_info_from_jar(path))
        .map(|res| res.expect("Could not extract mod metadata."))
        .collect::<Vec<_>>().await;

    Ok(mods)
}

// Update mods based on FS updates.
async fn sync_mods_from_fs(dir: &Path) -> crate::error::Result<HashMap<String, JARLoadedMod>> {
    let fs_mods = get_local_mods(dir).await?;
    let hash_map = fs_mods.into_iter()
        .map(|p| (p.file_hash.to_string(), p))
        .collect();

    Ok(hash_map)
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn retrieve_mod_metadata_test() -> Result<(), crate::error::Error> {
        // sha1 for Fabric version 0.67
        let fabric_sha1 = String::from("31174f7510f15ceddd3b449da4bffc0c2c589a4b");
        let hash_algo = String::from("sha1");
        let mod_name = String::from("Fabric");

        let jar_loaded_mod = retrieve_mod_metadata(&fabric_sha1, &hash_algo).await?;
        print!("{}", String::from(format!("{:?}", jar_loaded_mod)));
        assert_eq!(true, jar_loaded_mod.name.contains(&mod_name));
        Ok(())
    }
}