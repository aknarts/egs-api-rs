use webbrowser;
use std::io::{self};
use egs_api::EpicGames;
use egs_api::api::{EpicAsset, AssetInfo, Entitlement, Library, AssetManifest};
use std::collections::HashMap;
use std::collections::hash_map::{RandomState, Entry};

#[tokio::main]
async fn main() {
    if !webbrowser::open("https://www.epicgames.com/id/login?redirectUrl=https%3A%2F%2Fwww.epicgames.com%2Fid%2Fapi%2Fredirect").is_ok() {
        println!("Please go to https://www.epicgames.com/id/login?redirectUrl=https%3A%2F%2Fwww.epicgames.com%2Fid%2Fapi%2Fredirect")
    }
    println!("Please enter the 'sid' value from the JSON response");
    let mut sid = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut sid).unwrap();
    sid = sid.trim().to_string();
    sid = sid.replace(|c: char| c == '"', "");
    let mut egs = EpicGames::new();

    match egs.auth_sid(sid.as_str()).await {
        None => { println!("No exchange token, cannot login.") }
        Some(exchange_token) => {
            egs.auth_code(exchange_token).await;
            egs.login().await;
            let assets = egs.list_assets().await;
            let mut ueasset_map: HashMap<String, HashMap<String, EpicAsset>> = HashMap::new();
            let mut non_ueasset_map: HashMap<String, HashMap<String, EpicAsset>> = HashMap::new();
            for asset in assets {
                if asset.namespace == "ue" {
                    if !ueasset_map.contains_key(&asset.catalog_item_id.clone()) {
                        ueasset_map.insert(asset.catalog_item_id.clone(), HashMap::new());
                    };
                    match ueasset_map.get_mut(&asset.catalog_item_id.clone()) {
                        None => {
                        }
                        Some(old) => {
                            old.insert(asset.app_name.clone(), asset.clone());
                        }
                    };
                } else {
                    if !non_ueasset_map.contains_key(&asset.catalog_item_id.clone()) {
                        non_ueasset_map.insert(asset.catalog_item_id.clone(), HashMap::new());
                    };
                    match non_ueasset_map.get_mut(&asset.catalog_item_id.clone()) {
                        None => {
                        }
                        Some(old) => {
                            old.insert(asset.app_name.clone(), asset.clone());
                        }
                    };
                }
            }


            println!("Got {} assets", ueasset_map.len() + non_ueasset_map.len());
            println!("From that {} unreal assets", ueasset_map.len());
            println!("From that {} non unreal assets", non_ueasset_map.len());

            println!("Getting the asset metadata");
            egs.get_asset_metadata(ueasset_map.values().last().unwrap().values().last().unwrap().to_owned()).await;
            println!("Getting the asset info");
            egs.get_asset_info(ueasset_map.values().last().unwrap().values().last().unwrap().to_owned()).await;
            println!("Getting ownership token");
            egs.get_ownership_token(ueasset_map.values().last().unwrap().values().last().unwrap().to_owned()).await;
            println!("Getting the game token");
            egs.get_game_token().await;
            println!("Getting the entitlements");
            egs.get_user_entitlements().await;
            println!("Getting the library items");
            egs.get_library_items(true).await;
        }
    }
}