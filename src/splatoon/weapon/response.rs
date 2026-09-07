pub type RawResponse = Vec<RawWeaponInfo>;

#[derive(serde::Deserialize)]
pub struct RawWeaponInfo {
    key: String,
    aliases: Vec<String>,
    #[serde(rename = "type")]
    typ: RawWeaponType,
    name: RawWeaponName,
    matching_range: f32,
    main: String,
    sub: RawWeaponType,
    special: RawWeaponType,
    reskin_of: String,
}

#[derive(serde::Deserialize)]
pub struct RawWeaponType {
    key: String,
    aliases: Vec<String>,
    name: RawWeaponName,
}

#[derive(serde::Deserialize)]
pub struct RawWeaponName {
    en_US: String,
    ja_JP: String,
}
