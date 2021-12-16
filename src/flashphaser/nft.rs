

use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use std::collections::BTreeMap;


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct NftData {
    pub total_supply: u64,
    pub mint_flag: bool,
    pub adventure_gap: u64,//+++++++++++8小时
    pub tokens: BTreeMap<u64, Principal>,//人物编号 的拥有者(token_id -> user_id)
    pub controllers: Vec<Principal>,//该合约的管理员[Principal_1,Principal_2,Principal_3,....]
    pub claim_index: u64,//已经发行的数量
    //+++++++++++++++++++++++++
    pub mintlimit: BTreeMap<Principal, u64>,//每人mint的限制(Principal -> u64)
    pub xp: BTreeMap<u64, u64>,//(人物编号 => 经验值)
    pub level: BTreeMap<u64, u64>,//(人物编号 => mint与否)

    pub attribute_points: BTreeMap<u64, u64>,//(人物编号 => 属性点)
    pub attr_strength: BTreeMap<u64, u64>,//(人物编号 => 力量)
    pub attr_agility: BTreeMap<u64, u64>,//(人物编号 => 敏捷)
    pub attr_intelligence: BTreeMap<u64, u64>,//(人物编号 => 智力)

    pub adventurers_log: BTreeMap<u64, u64>,//(人物编号 => 冒险时间限定)
    //+++++++++++++++++++++++++++
    pub token_seeds: BTreeMap<u64, u64>
}


impl NftData {

    pub fn user_tokens(&self, user: &Principal) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();
        for (token_id, user_id) in &self.tokens {
            if user_id == user {
                results.push(*token_id);
            }
        }
        return results;
    }

    pub fn owner_of(&self, token_id: &u64) -> Option<Principal> {
        match self.tokens.get(token_id) {
            Some(owner_id) => return Some(owner_id.clone()),
            None => {
                return None;
            }
        }
    }

    pub fn is_owner_of(&self, user: Principal, token_id: &u64) -> bool {
        match self.owner_of(&token_id) {
            Some(owner_id) => return user == owner_id,
            None => {
                return false;
            }
        }
    }

    pub fn is_controller(&self, user: &Principal) -> bool {
        return self.controllers.contains(user);
    }

    pub fn add_controller(&mut self, user: &Principal) -> bool {
        if !self.is_controller(user) {
            self.controllers.push(user.clone());
            return true;
        }
        return false;
    }

    pub fn remove_controller(&mut self, user: &Principal) -> bool {
        if self.is_controller(user) {
            let index = self.controllers.iter().position(|x| x == user).unwrap();
            self.controllers.remove(index);
            return true;
        }
        return false;
    }

    pub fn transfer_to(&mut self, user: Principal, token_id: u64) -> bool {
        if let Some(token_owner) = self.tokens.get(&token_id) {
            if token_owner == &ic_cdk::caller() {
                self.tokens.insert(token_id, user);
                return true;
            }
        }
        return false;
    }

    pub fn remaining(&self) -> u64 {
        return self.total_supply - self.claim_index;
    }

    pub fn is_claimed(&self, token_id: &u64) -> bool {
        return self.tokens.contains_key(token_id);
    }

    //***********Claim一个英雄(注意初始化各个键值！！)********
    pub fn claim(&mut self, user_id: Principal) -> Result<u64, String> {
        //如果人数已满
        if self.claim_index >= self.total_supply {
            return Err("No more claims left".to_string());
        }
        match self.mintlimit.get(&user_id) {//如果该账户已mint过
            Some(_) => return Err("You have claimed one".to_string()),
            None => {
                self.claim_index += 1;
                //如果编号已占
                match self.tokens.get(&self.claim_index) {
                    Some(_) => return Err("This token already claimed".to_string()),
                    None => {
                        //生成种子(NFT id -> claimTime)
                        self.token_seeds.insert(self.claim_index, ic_cdk::api::time() as u64);
                        //记录到账本 (NFT编号 token_id -> 用户 user_id)
                        self.tokens.insert(self.claim_index, user_id);
                        self.level.insert(self.claim_index, 1 );//初始化等级
                        self.xp.insert(self.claim_index, 0 );//初始化经验值
                        self.adventurers_log.insert(self.claim_index, 0 );//初始化时间间隔
                        self.attribute_points.insert(self.claim_index, 6 );//初始化技能点

                        let seed = self.token_seeds.get(&self.claim_index).unwrap();
                        let _strength  = (seed  % 7 ) + 6 ;
                        let _agility  = (seed  % 11 ) + 2 ;
                        let _intelligence  = (seed  % 9 ) + 4 ;
                        self.attr_strength.insert(self.claim_index, _strength );//初始化力量
                        self.attr_agility.insert(self.claim_index, _agility );//初始化敏捷
                        self.attr_intelligence.insert(self.claim_index, _intelligence );//初始化智力

                        self.mintlimit.insert(user_id, 1);//写入mint标记
                        return Ok(self.claim_index);
                    }
                }
            }
        }
    }

    //【冒险】 参数：(&mut self,调用者pid,tokenID)
    pub fn adventure(&mut self, token_id: u64) -> Result<String, String> {
        
        if let Some(token_owner) = self.tokens.get(&token_id) {//获取 token_id 的拥有者(如果有的话)
            if token_owner == &ic_cdk::caller() {//如果 token_id 的拥有者 等于 该方法的调用者
                let mut _timelimit = ic_cdk::api::time() as u64;
                ic_cdk::println!("_timelimit , {:?}", _timelimit.to_string());
                if *self.adventurers_log.get(&token_id).unwrap() < _timelimit{

                    let mut _xp = self.xp.get(&token_id).unwrap();//* 是转&u64为u64?  
                    let mut _level = self.level.get(&token_id).unwrap();//* 是转&u64为u64?
                    let mut _points = self.attribute_points.get(&token_id).unwrap();//* 是转&u64为u64?

                    self.adventurers_log.insert(token_id, _timelimit + self.adventure_gap);//如果不存在此键，None则返回。如果地图确实存在此键，则更新该值，并返回旧值。
                    let mut _xp2  = _xp + 150;//_xp 如果是&u64 ，则相加也需要&u64 .但150是u64 ,所以要么现在转*，或者提前转*
                    if _xp2 >= 1000{
                        _xp2  = _xp2 - 1000;
                        let mut _level2 =  _level + 1;
                        let mut _points2 =  _points + 3;
                        self.level.insert(token_id, _level2 );
                        self.attribute_points.insert(token_id, _points2);
                    }
                    self.xp.insert(token_id, _xp2 );
                    
                    return Ok("Adventure complete ! ".to_string());
                }else{
                    return Err("The interval hasn't come yet. have a rest".to_string());
                }
            }else{
                return Err("You are not the NFT owner!".to_string());
            }
        }else{
            return Err("Nobody own this NFT.".to_string());
        }

    }










    //0000000000***********Claim一个英雄(注意初始化各个键值！！)********
    pub fn claim000(&mut self, user_id: Principal) -> Result<u64, String> {
        //如果人数已满
        if self.claim_index >= self.total_supply {
            return Err("No more claims left".to_string());
        }
        self.claim_index += 1;
        //如果编号已占
        match self.tokens.get(&self.claim_index) {
            Some(_) => return Err("this token already claimed".to_string()),
            None => {
                    //生成种子(NFT id -> claimTime)
                    self.token_seeds.insert(self.claim_index, ic_cdk::api::time() as u64);
                    //记录到账本 (NFT编号 token_id -> 用户 user_id)
                    self.tokens.insert(self.claim_index, user_id);
                    self.level.insert(self.claim_index, 1 );//初始化等级
                    self.xp.insert(self.claim_index, 0 );//初始化经验值
                    self.adventurers_log.insert(self.claim_index, 0 );//初始化时间间隔
                    self.attribute_points.insert(self.claim_index, 6 );//初始化技能点

                    let seed = self.token_seeds.get(&self.claim_index).unwrap();
                    let _strength  = (seed  % 7 ) + 6 ;
                    let _agility  = (seed  % 11 ) + 2 ;
                    let _intelligence  = (seed  % 9 ) + 4 ;
                    self.attr_strength.insert(self.claim_index, _strength );//初始化力量
                    self.attr_agility.insert(self.claim_index, _agility );//初始化敏捷
                    self.attr_intelligence.insert(self.claim_index, _intelligence );//初始化智力

                    self.mintlimit.insert(user_id, 1);//写入mint标记
                    return Ok(self.claim_index);

            }
        }
    }





    //【冒险】 参数：(&mut self,调用者pid,tokenID)
    pub fn adventure000(&mut self, token_id: u64) -> Result<String, String> {
        
        if let Some(token_owner) = self.tokens.get(&token_id) {//获取 token_id 的拥有者(如果有的话)
            if token_owner == &ic_cdk::caller() {//如果 token_id 的拥有者 等于 该方法的调用者
                let mut _timelimit = ic_cdk::api::time() as u64;
                ic_cdk::println!("_timelimit , {:?}", _timelimit.to_string());
                if *self.adventurers_log.get(&token_id).unwrap() != _timelimit{

                    let mut _xp = self.xp.get(&token_id).unwrap();//* 是转&u64为u64?  
                    let mut _level = self.level.get(&token_id).unwrap();//* 是转&u64为u64?  
                    let mut _points = self.attribute_points.get(&token_id).unwrap();//* 是转&u64为u64?
                    
                    self.adventurers_log.insert(token_id, _timelimit + self.adventure_gap);//如果不存在此键，None则返回。如果地图确实存在此键，则更新该值，并返回旧值。
                    let mut _xp2  = _xp + 350;//_xp 如果是&u64 ，则相加也需要&u64 .但150是u64 ,所以要么现在转*，或者提前转*
                    if _xp2 >= 1000{
                        _xp2  = _xp2 - 1000;
                        let mut _level2 =  _level + 1;
                        let mut _points2 =  _points + 3;
                        self.level.insert(token_id, _level2 );
                        self.attribute_points.insert(token_id, _points2);
                    }
                    self.xp.insert(token_id, _xp2 );
                    
                    return Ok("Adventure complete ! ".to_string());
                }else{

                    return Err("The interval hasn't come yet. have a rest".to_string());
                }
            }else{
                return Err("You are not the NFT owner!".to_string());
            }
        }else{
            return Err("Nobody own this NFT.".to_string());
        }

    }

    pub fn add_points(&mut self, token_id: u64,attribute: u64,amounts:u64) -> Result<String, String> {
            
        if let Some(token_owner) = self.tokens.get(&token_id) {//获取 token_id 的拥有者(如果有的话)
            if token_owner == &ic_cdk::caller() {//如果 token_id 的拥有者 等于 该方法的调用者
                let mut _points = *self.attribute_points.get(&token_id).unwrap();
                if (_points >= amounts) && (_points > 0) {//如果 token_id 的属性点大于0
                    _points  = _points - amounts;
                    if attribute  == 0{
                        self.attribute_points.insert(token_id, _points);
                        self.attr_strength.insert(token_id, amounts);
                        return Ok("Attributes alloc sucess ! ".to_string());
                    }else if attribute  == 1{
                            self.attribute_points.insert(token_id, _points);
                            self.attr_intelligence.insert(token_id, amounts);
                        return Ok("Attributes alloc sucess ! ".to_string());
                    }else if  attribute  == 2{
                            self.attribute_points.insert(token_id, _points);
                            self.attr_intelligence.insert(token_id, amounts);
                            return Ok("Attributes alloc sucess ! ".to_string());
                    }else{
                        return Err("Please input true code !".to_string());
                    }
                    
                
                }else{
                    return Err("Your Attributes Points not enough !".to_string());
                }
            }else{
                return Err("You are not the NFT owner!".to_string());
            }
        }else{
            return Err("Nobody own this NFT.".to_string());
        }

    }


    //////////////////////////////
    pub fn mint_on(&mut self) -> bool {
        self.mint_flag = true;
        return self.mint_flag;
    }

    pub fn mint_off(&mut self) -> bool {
        self.mint_flag = false;
        return self.mint_flag;
    }

    pub fn get_mint_flag(&mut self) -> bool {
        return self.mint_flag;
    }
    //////////////////////////////

    pub fn querygap(&mut self, token_id: u64) -> u64 {
        return *self.adventurers_log.get(&token_id).unwrap();
    }

    pub fn queryxp(&mut self, token_id: u64) -> u64 {
            return *self.xp.get(&token_id).unwrap();
    }

    pub fn querypoint(&mut self, token_id: u64) -> u64 {
        return *self.attribute_points.get(&token_id).unwrap();
    }

    pub fn querylevel(&mut self, token_id: u64) -> u64 {
        return *self.level.get(&token_id).unwrap();
    }

    pub fn querycaller(&mut self,user_id: Principal) -> Option<Principal> {
        ic_cdk::println!("Caller is ->  {:?}", user_id.to_string());
        return Some(user_id.clone());
    }

    //++++++++++++++++++++++++++++++++++++++++


}
