use crate::{Hitokoto, HitokotoCat, HitokotoCatList, HitokotoSegment};

#[derive(Deserialize)]
struct RawHitokotoCat {
    id: u32,
    name: String,
    desc: String,
    path: std::path::PathBuf,
}

/// Get `Hitokoto` from
/// [hitokoto-osc/sentences-bundle](https://github.com/hitokoto-osc/sentences-bundle/)
pub fn get_from_raw(path: std::path::PathBuf) -> Hitokoto {
    use serde_json::from_str;
    use std::fs;
    let categories: Vec<RawHitokotoCat> =
        from_str(&fs::read_to_string(path.join("categories.json")).unwrap()).unwrap();
    let mut hitokoto: Vec<HitokotoCatList> = vec![];
    for cat in categories {
        let mut list: Vec<HitokotoSegment> =
            from_str(&fs::read_to_string(path.join(cat.path)).unwrap()).unwrap();
        let mut i = 1;
        for segment in &mut list {
            segment.id = i;
            i = i + 1;
        }
        hitokoto.push(HitokotoCatList {
            cat: HitokotoCat {
                id: cat.id,
                name: cat.name,
                desc: cat.desc,
            },
            list,
        });
    }

    Hitokoto { list: hitokoto }
}
