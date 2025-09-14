use rig::Embed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Embed)]
pub struct Chunk {
    pub source_id: String,
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

    pub fn chunk_text(&self, text: &str, source_id: String) -> Vec<Chunk> {
        let mut chunks: Vec<Chunk> = Vec::new();
        let tokens = text.split_whitespace();
        let real_chunk_size = self.chunk_size - self.overlap_size;
        let num_chunks = (tokens.clone().count() + real_chunk_size - 1) / real_chunk_size;
        println!("Num chunks: {:?}", num_chunks);
        println!("Chunk size: {:?}", self.chunk_size - self.overlap_size);
        println!("Tokens: {:?}", tokens.clone().count());
        for i in 0..num_chunks {
            let start_index = i * real_chunk_size;
            let end_index = std::cmp::min(start_index + self.chunk_size, tokens.clone().count());
            if tokens.clone().count() > end_index {
                chunks.push(Chunk {
                    source_id: source_id.clone(),
                    text: tokens.clone().skip(start_index).take(end_index - start_index).collect::<Vec<&str>>().join(" "),
                });
            } else {
                let remaining_tokens = tokens.clone().count() - start_index;
                chunks.push(Chunk {
                    source_id: source_id.clone(),
                    text: tokens.clone().skip(start_index).take(remaining_tokens).collect::<Vec<&str>>().join(" "),
                });
            }
        }
        chunks
    }
}
