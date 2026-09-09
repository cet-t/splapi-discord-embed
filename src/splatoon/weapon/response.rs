pub type RawResponse = Vec<RawWeaponInfo>;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct RawWeaponInfo {
    key: String,
    aliases: Vec<String>,
    #[serde(rename = "type")]
    typ: RawWeaponType,
    pub name: RawWeaponName,
    matching_range: f32,
    main: String,
    sub: RawWeaponType,
    special: RawWeaponType,
    reskin_of: String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct RawWeaponType {
    key: String,
    aliases: Vec<String>,
    name: RawWeaponName,
}

#[allow(dead_code, non_snake_case)]
#[derive(serde::Deserialize)]
pub struct RawWeaponName {
    en_US: String,
    pub ja_JP: String,
}
