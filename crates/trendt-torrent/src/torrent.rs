use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    /// Primary tracker URL
    pub announce: String,

    /// Optional: backup tracker URLs (list of tiers, each tier is a list of URLs)
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,

    /// Optional: Unix timestamp when torrent was created
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,

    /// Optional: free-form comment
    pub comment: Option<String>,

    /// Optional: client that created this torrent
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}
