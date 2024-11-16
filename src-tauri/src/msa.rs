use serde::Serialize;

#[derive(Serialize)]
pub struct MsaResult {
    pub aligned_sequences: Vec<String>,
    pub consensus_sequence: String,
}

pub fn perform_msa(sequences: Vec<String>) -> MsaResult {
    let aligned_sequences = vec![
        "ATGC".to_string(),
        "A-GC".to_string(),
        "AT-C".to_string(),
    ];
    let consensus_sequence = "ATGC".to_string();

    MsaResult {
        aligned_sequences,
        consensus_sequence,
    }
}