use hdk::prelude::*;

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadataInput {
    pub file_name: String,
    pub file_size: usize,
    pub file_type: String,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum PayloadInput {
    #[serde(rename_all = "camelCase")]
    Text { payload: String },
    #[serde(rename_all = "camelCase")]
    File {
        metadata: FileMetadataInput,
        file_type: FileType,
        file_bytes: SerializedBytes,
    },
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub file_name: String,
    pub file_size: usize,
    pub file_type: String,
    pub file_hash: EntryHash,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum FileType {
    Image { thumbnail: SerializedBytes },
    Video { thumbnail: SerializedBytes },
    Other,
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum Payload {
    #[serde(rename_all = "camelCase")]
    Text { payload: String },
    #[serde(rename_all = "camelCase")]
    File {
        metadata: FileMetadata,
        file_type: FileType,
    },
}

#[derive(Serialize, Deserialize, SerializedBytes, Clone, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE", content = "payload")]
pub enum PayloadType {
    Text,
    File,
    Media,
    All,
}
