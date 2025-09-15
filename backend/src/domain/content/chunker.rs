use rig::Embed;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::domain::sources::models::Source;

#[derive(Debug, Clone, Serialize, Deserialize, Embed)]
pub struct Chunk {
    pub source_id: String,
    pub source_url: String,
    pub source_title: String,
    #[embed]
    pub text: String,
}

// content/chunker.rs
pub struct TextChunker {
    pub chunk_size: usize,    // 400 токенов
    pub overlap_size: usize,  // 100 токенов
}

impl TextChunker {
    pub fn new(chunk_size: usize, overlap_size: usize) -> Self {
        Self {
            chunk_size,
            overlap_size,
        }
    }

    pub fn chunk_text_from_source(&self, source: &Source) -> Vec<Chunk> {
        let mut chunks: Vec<Chunk> = Vec::new();
        let source_id = source.id.clone().unwrap_or_default().to_string();
        let source_url = source.url.clone();
        let source_title = source.title.clone();
        let tokens = source.raw_text.split_whitespace();
        let real_chunk_size = self.chunk_size - self.overlap_size;
        let num_chunks = (tokens.clone().count() + real_chunk_size - 1) / real_chunk_size;
        info!("Creating {} chunks from {} tokens", num_chunks, tokens.clone().count());
        for i in 0..num_chunks {
            let start_index = i * real_chunk_size;
            let end_index = std::cmp::min(start_index + self.chunk_size, tokens.clone().count());
            if tokens.clone().count() > end_index {
                chunks.push(Chunk {
                    source_id: source_id.clone(),
                    text: tokens.clone().skip(start_index).take(end_index - start_index).collect::<Vec<&str>>().join(" "),
                    source_url: source_url.clone(),
                    source_title: source_title.clone(),
                });
            } else {
                let remaining_tokens = tokens.clone().count() - start_index;
                chunks.push(Chunk {
                    source_id: source_id.clone(),
                    text: tokens.clone().skip(start_index).take(remaining_tokens).collect::<Vec<&str>>().join(" "),
                    source_url: source_url.clone(),
                    source_title: source_title.clone(),
                });
            }
        }
        chunks
    }
}
