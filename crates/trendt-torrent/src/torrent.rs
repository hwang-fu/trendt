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

    /// File metadata and piece hashes
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    /// File or directory name
    pub name: String,

    /// Number of bytes per piece
    #[serde(rename = "piece length")]
    pub piece_length: i64,

    /// Concatenated SHA-1 hashes (20 bytes each)
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,

    /// File size in bytes (single-file torrents only)
    pub length: Option<i64>,
}
