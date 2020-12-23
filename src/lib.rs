//! hitokoto-rust
//!
//! A hitokoto server writeen in rust.
#[macro_use]
extern crate serde_derive;

use std::convert::{TryFrom, TryInto};

pub mod from_raw;
pub mod pool;
pub mod server;

#[derive(Serialize)]
pub struct HitokotoCat {
    id: u32,
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct HitokotoSegment {
    id: u32,
    uuid: String,
    hitokoto: String,
    from: String,
}

pub struct HitokotoCatList {
    cat: HitokotoCat,
    list: Vec<HitokotoSegment>,
}

pub struct Hitokoto {
    list: Vec<HitokotoCatList>,
}

#[derive(Serialize)]
pub enum HitokotoError {
    CatIDNotFound(u32),
    HikokotoIDNotFound(u32),
    FailedParse(u32),
}

impl HitokotoCat {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_desc(&self) -> &str {
        &self.desc
    }
}

impl HitokotoCatList {
    /// Get `HitokotoSegment` from given id
    pub fn get_hitokoto(&self, raw_id: u32) -> Result<&HitokotoSegment, HitokotoError> {
        let id = usize::try_from(raw_id);
        if id.is_err() {
            return Err(HitokotoError::FailedParse(raw_id));
        }
        let id = id.unwrap();
        match self.list.get(id - 1) {
            Some(segment) => Ok(segment),
            None => Err(HitokotoError::HikokotoIDNotFound(raw_id)),
        }
    }
    /// Get all hitokoto as a vector list.
    pub fn get_hitokoto_list(&self) -> Vec<&HitokotoSegment> {
        let mut list = vec![];
        for hitokoto in &self.list {
            list.push(hitokoto);
        }

        list
    }
}

impl HitokotoCatList {
    /// Get cat details
    pub fn get_cat(&self) -> &HitokotoCat {
        &self.cat
    }
}

impl Hitokoto {
    /// Get all cat
    pub fn get_cat_list(&self) -> Vec<&HitokotoCat> {
        let mut result = vec![];
        for hitokoto_cat in &self.list {
            let hitokoto_cat = hitokoto_cat.get_cat();
            result.push(hitokoto_cat);
        }

        result
    }
    /// Get `HitokotoCat` from given cat
    pub fn get_cat_hitokoto(&self, id: u32) -> Result<&HitokotoCatList, HitokotoError> {
        for hitokoto_cat in &self.list {
            if hitokoto_cat.get_cat().get_id() == id {
                return Ok(hitokoto_cat);
            }
        }

        Err(HitokotoError::CatIDNotFound(id))
    }
    /// Get `HitokotoSegment` from given cat and id.
    /// When id = 0, it will get a random value.
    pub fn get_hitokoto(
        &self,
        cat_id: u32,
        hitokoto_id: u32,
    ) -> Result<&HitokotoSegment, HitokotoError> {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut cat_id = cat_id;
        let mut hitokoto_id = hitokoto_id;
        if cat_id == 0 {
            cat_id = rng.gen_range(1..self.get_cat_list().len().try_into().unwrap());
        }
        let cat_list = self.get_cat_hitokoto(cat_id)?;
        if hitokoto_id == 0 {
            hitokoto_id =
                rng.gen_range(0..cat_list.get_hitokoto_list().len().try_into().unwrap()) + 1;
        }
        cat_list.get_hitokoto(hitokoto_id)
    }
}
