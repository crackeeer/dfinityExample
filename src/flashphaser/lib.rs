use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde_bytes::{ByteBuf};
use std::collections::BTreeMap;

mod rush;
mod rand;
mod nft;

use crate::nft::NftData;
use crate::rush::Rush;

//初始化
#[init]
fn init() {
    ic_cdk::println!("Init {:?}", ic_cdk::api::time());
    let rush_nft = storage::get_mut::<NftData>();
    rush_nft.total_supply = 10000;//总供应量
    rush_nft.mint_flag = false;
    rush_nft.adventure_gap = 8*3600*1000000000;//冒险时间间隔
    //needs to be a way to pass this into init
    let owner_id = Principal::from_text(
        "7jk22-6zyr6-ufepb-bcitn-zqdhf-4lkel-ls3qn-4yydx-ozkvy-f3mas-pqe"
    ).unwrap();
    rush_nft.add_controller(&owner_id);

    init_rush();
}

fn init_rush() -> () {

    //实例化rush,并进行初始化
    let rush = storage::get_mut::<Rush>();
    //一些静态的属性，写入到rush中。
    rush.weapons = vec![
        "Iron Sword",
        "Bronze Sword",
        "Silver Sword",
        "Fire Sword",
        "Wind Sword",
        "Iron Dagger",
        "Bronze Dagger",
        "Silver Dagger",
        "Crystal Dagger",
        "Blink Dagger",
        "Iron Axe",
        "Bronze Axe",
        "Silver Axe",
        "Ogre Axe",
        "Fury Axe🪓",
        "Iron Lance",
        "Silver Lance",
        "Dragon Lance",
        "Fire Javelin",
        "Iron Hammer",
        "Bronze  Hammer",
        "Meteor Hammer",
        "Thunder Hammer🔨",
        "Wooden Wand",
        "Cooper Wand",
        "Broken Wand",
        "Silver Wand",
        "Demon Wand",
        "Witch Blade",
        "Crystalys",
        "Shadow Blade",      
        "Ethereal Blade",
        "Monkey King Bar",
        "Daedalus Blade",
        "Abyssal Blade",
        "Divine Rapier",
        "Echo Sabre",
        "Yasha Katana",
        "Heaven's Wrench🔧",
        "satanic Trident🔱",
    ].iter().map(|s| s.to_string()).collect();

    rush.chest = vec![
        "Protection Shirt",
        "Venom Hoodie",
        "Fishnet Shirt",
        "Plastic Shirt",
        "Polyester Shirt",
        "Hawaiian Shirt",
        "Polo Shirt",
        "Ring Mail",
        "Plate mail",
        "Blade Mail",
        "Storm Mail",
        "Crimson Guard",
        "Assault Cuirass",
        "Bowling Shirt",
        "Wind Shawl",
        "Falcon Cardigan",
        "Rain Coat",
        "Graphic Tee",
        "Shadow Tunic",
        "Gold Corset",
        "Dominator Sweater",
        "Crop Top",
        "Grey Cardigan",
        "Safari Jacket",
        "Bikini Top",
        "Ghost Dress",
        "Eternal Robe",
        "Puffer Jacket",
        "Iron Jacket",
        "Fire Jacket",
        "Dragon Coat",
        "Vitality Tailcoat",
        "Tuxedo Jacket",
        "Energy Parka",
        "Ethereal Hanfu",
        "Demon Overcoat",
        "Turtle Neck",
        "Bomber Jacket",
        "Trench Coat",
        "Eaglesong Cape",
        "Void Kimono",
    ].iter().map(|s| s.to_string()).collect();

    rush.head = vec![
        "Iron Helmet",
        "Bronze Helmet",
        "Silver Helmet",
        "Gold Helmet",
        "Blitz Helmet",
        "Sun Hat",
        "Viking Helmet",
        "Silk Hood",
        "Fluffy Hat",
        "Voodoo Mask",
        "Bucket Hat",
        "Baseball Cap",
        "Morbid Mask",
        "Helm of the Overlord",
        "Porkpie Hat",
        "Panama Hat",
        "Sailors Bonet",
        "Silver Beanie",
        "Gold Headband",
        "Captain Hat",
        "Equestrian Helm",
        "Dragon Visor",
        "Tiger Turban",
        "Wedding Veil",
        "Feather Hat",
        "Propeller Hat",
        "Cat Ears",
        "Headphones",
        "Broken Helmet",
        "Wizard Hat",
        "Safari Hat",
        "Pith Helmet",
        "Ice Tiara",
        "Flower Crown",
        "Fire Crown",
        "Umbrella Hat",
        "Motorcycle Helmet",
        "Peaked Cap",
        "Golf Visor",
        "Bowler Hat",
        "Bow Hat",
        "Dunce Cap",
        "Flash Towel",
        "Jewelry Headpiece",
        "Yarmulke",
        "Bald Cap",
        "Coonskin Cap",
        "Balaclava",
        "Bonnet",
        "Fez",
        "Space Helmet",
        "Veil of Discord",
        "Hood of Defiance",
    ].iter().map(|s| s.to_string()).collect();
    rush.accessory = vec![
        "Tome of Knowledge",
        "Gauntlets of Strength",
        "Orb of Venom",
        "Pearl Chain",
        "Iron Choker",
        "Cross Necklace",
        "Shadow Amulet",
        "Energy Gem",
        "Neck Tie",
        "Talisman of Evasion",
        "Bracer",
        "Scarf",
        "Feather Boa",
        "Neck Pillow",
        "Ascot",
        "Hope Diamond",
        "Ring of Basilius",
        "Stethoscope",
        "Shades",
        "Iron Buckler",
        "Nerd Glasses",
        "Wooden Buckler",
        "Gold Buckler",
        "Contact Lenses",
        "Monocle",
        "Green Contacts",
        "Fire Pipe",
        "Sunglasses",
        "Blindfold",
        "Soul Booster",
        "Cyclops Glasses",
        "Aviator Sunglasses",
        "Spectacles",
        "Blood stone",
        "Backpack",
        "Grocery Bag",
        "Briefcase",
        "Suitcase",
        "Messenger Bag",
        "Lotus Orb",
        "Crossbody Bag",
        "Hat Bag",
        "Golf Bag",
        "Portfolio Bag",
        "Tackle Box",
        "Arcane Ring",
        "Steamer Trunk",
        "Tool Box",
        "Guitar Case",
        "Watch",
        "Smart Watch",
        "Bullwhip",
        "Atomic Watch",
        "Earth Vambrace",
        "Ice Vambrace",
        "Fire Vambrace",
        "Wind Vambrace",
        "Engagement Ring",
        "Antique Watch",
        "Wristband",
        "Scrunchie",
        "Aquila Ring",
        "Vambrace",
        "Ring of Storm",
        "Ring of Thunder",
        "Ring of Flash",
        "Ring of Cloud",
    ].iter().map(|s| s.to_string()).collect();

    rush.classes = vec![
        "Warrior",
        "Paladin",
        "Hunter",
        "Rogue",
        "Priest",
        "Shaman",
        "Wizard",
        "Warlock",
        "Monk",
        "Druid",
        "Warrior",
        "Paladin",
        "Hunter",
        "Rogue",
        "Priest",
        "Shaman",
        "Wizard",
        "Warlock",
        "Monk",
        "Druid",
        "Demon Hunter👻",
        "Death Knight🎃",
    ].iter().map(|s| s.to_string()).collect();

    rush.races = vec![
        "Human",
        "Elf",
        "Orc",
        "Gnome",
        "Worgen",
        "Troll",
        "Undead",
        "Blood Elf",
        "Pandaren", 
    ].iter().map(|s| s.to_string()).collect();
    
    rush.faction = vec![
        "Radiant",
        "Dire",
        "Neutral",
    ].iter().map(|s| s.to_string()).collect();



   

    rush.pants = vec![
        "Shorts",
    ].iter().map(|s| s.to_string()).collect();

    rush.prefixes = vec![
        "Ancient",
        "Broken",
        "Futuristic",
        "Shining",
        "Magical",
        "Dim",
        "Second Hand",
        "Aged",
        "Victorian",
        "Worn Out",
        "Brand New",
        "Used",
        "Beautiful",
        "Old",
        "Expensive",
        "Synthetic",
        "Monogramed",
        "Medieval",
        "Vintage",
        "Retro",
    ].iter().map(|s| s.to_string()).collect();
    
////////////////////////////////////
    rush.name_prefixes = vec![
        "Gold",
    ].iter().map(|s| s.to_string()).collect();

    rush.name_suffixes = vec![
        "Clean",
    ].iter().map(|s| s.to_string()).collect();
}


//暴露于外的接口，实际是要和 NftData 通信
#[query]
fn user_tokens(user: Principal) -> Vec<u64> {
    return storage::get::<NftData>().user_tokens(&user);
}

#[query]
fn supply() -> u64 {
    return storage::get::<NftData>().total_supply;
}

#[query]
fn remaining() -> u64 {
    return storage::get::<NftData>().remaining();
}

#[query]
fn owner_of(token_id: u64) -> Option<Principal> {
   return storage::get::<NftData>().owner_of(&token_id);
}

#[update]
fn transfer_to(user: Principal, token_id: u64) -> bool {
    return storage::get_mut::<NftData>().transfer_to(user, token_id);
}

#[update]
fn claim() -> Result<u64, String> {
    if storage::get_mut::<NftData>().mint_flag {
      return storage::get_mut::<NftData>().claim(ic_cdk::caller());
    }
    return Err("How dare you! the ic-Rush not for claim".to_string());
    
}


//+++++++++++++++++++++++++++++++++++++++++++++++++++++++
#[update]
fn adventure(token_id: u64) -> Result<String, String>  {
    return storage::get_mut::<NftData>().adventure(token_id);
}

#[update]
fn add_points(token_id: u64,attribute: u64,amounts:u64) -> Result<String, String>  {
    return storage::get_mut::<NftData>().add_points(token_id,attribute,amounts);
}
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[update(guard = "is_controller")]
fn adventure000(token_id: u64) -> Result<String, String>  {
    return storage::get_mut::<NftData>().adventure000(token_id);
}

#[update(guard = "is_controller")]
fn claim000() -> Result<u64, String> {
    //return Err("No claims for this NFT type (IC DRIP)".to_string());
    return storage::get_mut::<NftData>().claim000(ic_cdk::caller());
}
//////////////////////////////////
#[update(guard = "is_controller")]
fn mint_on() -> bool  {
    return storage::get_mut::<NftData>().mint_on();
}

#[update(guard = "is_controller")]
fn mint_off() -> bool  {
    return storage::get_mut::<NftData>().mint_off();
}

#[query]
fn get_mint_flag() -> bool  {
    return storage::get_mut::<NftData>().get_mint_flag();
}
//////////////////////////////////

#[query]
pub fn queryxp(token_id: u64) -> u64 {
    return storage::get_mut::<NftData>().queryxp(token_id);
}


#[query]
pub fn querygap(token_id: u64) -> u64 {
    return storage::get_mut::<NftData>().querygap(token_id);
}

#[query]
pub fn querypoint(token_id: u64) -> u64 {
    return storage::get_mut::<NftData>().querypoint(token_id);
}

#[query]
pub fn querylevel(token_id: u64) -> u64 {
    return storage::get_mut::<NftData>().querylevel(token_id);
}

#[query]
pub fn querycaller() -> Option<Principal> {
    return storage::get_mut::<NftData>().querycaller(ic_cdk::caller());
}
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++

//Allow the original airdrop to always exists for future references
//where sites can use this to know if the person transferred their NFT or not.
#[query]
fn get_airdrops() -> Vec<(u64, bool)> {
    let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
    let rush_nft = storage::get_mut::<NftData>();
    match airdroppers.get(&ic_cdk::caller()) {
        Some(tokens) => {
            let mut results: Vec<(u64, bool)> = Vec::new();
            for token in tokens {
                results.push((
                    token.clone(), 
                    rush_nft.is_owner_of(ic_cdk::caller(), token)
                ));
            }
            return results;
        },
        None => Vec::new()
    }
}

//Save list of airdrops for other platforms to use.
fn update_airdroppers(user: Principal, token_id: u64) -> () {
    let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
    match airdroppers.get_mut(&user) {
        Some(tokens) => tokens.push(token_id),
        None => {
            airdroppers.insert(user, vec![token_id]);
        }
    }
}

#[update(guard = "is_controller")]
fn add_airdrops(users: Vec<Principal>) -> bool {
    let rush_nft = storage::get_mut::<NftData>();
    for id in users {
        match rush_nft.claim(id) {
            Ok(token_id) => update_airdroppers(id, token_id),
            Err(_) => return false
        }
    }
    return true;
}

#[update(guard = "is_controller")]
fn add_controller(user: Principal) -> bool {
    return storage::get_mut::<NftData>().add_controller(&user);
}

#[update(guard = "is_controller")]
fn remove_controller(user: Principal) -> bool {
    return storage::get_mut::<NftData>().remove_controller(&user);
}

#[update(guard = "is_controller")]
fn get_controllers() -> Vec<Principal> {
    return storage::get::<NftData>().controllers.clone();
}

#[query]
fn name() -> String {
    return "IC_RUSH".to_string();
}

#[query]
fn symbol() -> String {
    return "IC_RUSH".to_string();
}

#[query]
fn get_nft_data() -> NftData {
    return storage::get::<NftData>().clone();
}

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
}

//发起网络请求后：
#[query]
async fn http_request(req: HttpRequest) -> HttpResponse {

    let parts: Vec<&str> = req.url.split('?').collect();

    let token_param: Vec<&str> = parts[1].split('=').collect();
    let token_id = token_param[1].parse::<u64>().unwrap();

    let rush_nft = storage::get_mut::<NftData>();

    if token_id <= 0 || token_id > rush_nft.total_supply || !rush_nft.is_claimed(&token_id) {
        return HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    let rush = storage::get_mut::<Rush>();

    // 获取指定的NFT 的生成种子
    let seed = rush_nft.token_seeds.get(&token_id).unwrap();
    //将指定的NFT生成种子进行加工（token_id + seed 这个数值）传递给generate()
    //为什么传递这么大的值？ + seed.clone()
    let data = rush.generate(token_id.clone());
    //将generate返回的data (一些html)进行序列化
    let results = data.as_bytes();
    //组装成http respon
    let mut headers: Vec<HeaderField> = Vec::new();
    headers.push(("content-type".to_string(), "image/svg+xml".to_string()));
    headers.push(("cache-control".to_string(), "public, max-age=604800, immutable".to_string()));
    return HttpResponse {
        status_code: 200,
        headers,
        body: results.to_vec(),
    }
}


// 获取NFT属性，形如("hand".to_string(), self.get_weapon(token_id))的数组
#[query]
fn get_token_properties(token_id: u64) -> Vec<(String, String)> {

    let rush_nft = storage::get_mut::<NftData>();
    if token_id <= 0 || token_id > rush_nft.total_supply || !rush_nft.is_claimed(&token_id) {
        return Vec::new();
    }

    let rush = storage::get_mut::<Rush>();
    return rush.get_properties(token_id);
    //let mut prop = rush.get_properties(token_id);
    //prop.push(("aaa".to_string(),rush_nft.xp.get(&token_id).unwrap().to_string()));
    //return prop;

}


//this is not working correctly.
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    return r#"
type HeaderField = record { text; text; };

type HttpRequest = record {
    method: text;
    url: text;
    headers: vec HeaderField;
    body: blob;
};

type HttpResponse = record {
    status_code: nat16;
    headers: vec HeaderField;
    body: blob;
};

type NftData = record {
    total_supply: nat64;
    tokens: vec record { nat64; principal};
    controllers: vec principal;
    claim_index: nat64;
    token_seeds: vec record { nat64; nat64};
};

type ClaimResult = variant {
    Ok : nat64;
    Err: text;
};

service : {
    http_request: (request: HttpRequest) -> (HttpResponse) query;

    get_nft_data: () -> (NftData) query;
    get_token_properties: (nat64) -> (vec record { text; text}) query;

    get_airdrops: () -> (vec record { nat64; bool }) query;
    add_airdrops: (vec principal) -> (bool);
    name: () -> (text) query;
    symbol: () -> (text) query;
    user_tokens: (principal) -> (vec nat64) query;
    owner_of: (nat64) -> (opt principal) query;
    transfer_to: (principal, nat64) -> (bool);
    claim: () -> (ClaimResult);
    remaining: () -> (nat64);

    get_controllers: () -> (vec principal) query;
    add_controller: (principal) -> (bool);
    remove_controller: (principal) -> (bool);
    supply: () -> (nat64);
}
    "#.to_string();
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    rush_nft: NftData,
    airdroppers: BTreeMap<Principal, Vec<u64>>
}


#[pre_upgrade]
fn pre_upgrade() {

    let stable = StableStorage {
        rush_nft: storage::get::<NftData>().clone(),
        airdroppers: storage::get::<BTreeMap<Principal, Vec<u64>>>().clone(),
    };

    match storage::stable_save((stable,)) {
        Ok(_) => (),
        Err(candid_err) => {
            ic_cdk::trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
fn post_upgrade() {
    init();
    if let Ok((storage,)) = storage::stable_restore::<(StableStorage,)>() {

        let rush_nft = storage::get_mut::<NftData>();
        *rush_nft = storage.rush_nft;
        
        let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
        *airdroppers = storage.airdroppers;
    }
}

fn is_controller() -> Result<(), String> {
    if storage::get::<NftData>().is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        Err("Only the controller can call this method.".to_string())
    }
}
