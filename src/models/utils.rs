use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Serializer};

pub fn serialize_oid<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(oid) => serializer.serialize_str(&oid.to_hex()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_oid_vec<S>(ids: &Vec<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let strings: Vec<String> = ids.iter().map(|oid| oid.to_hex()).collect();
    strings.serialize(serializer)
}
