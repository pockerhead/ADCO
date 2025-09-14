-- ADCO Initial Database Schema
-- Enables pgvector extension and creates all required tables

-- Enable pgvector extension for embeddings
CREATE EXTENSION IF NOT EXISTS vector;

-- Generate UUID extension for posts
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Sources table: raw documents from search/RSS/articles
CREATE TABLE sources (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  url TEXT,
  title TEXT,
  source_type TEXT,
  fetched_at TIMESTAMPTZ,
  raw_text TEXT
);

-- Chunks table: text chunks + embeddings for RAG
CREATE TABLE chunks (
  id SERIAL PRIMARY KEY,
  source_id UUID REFERENCES sources(id),
  text TEXT NOT NULL,
  embedding VECTOR(1536),
  meta JSONB,
  created_at TIMESTAMPTZ DEFAULT now()
);

-- Index for fast vector similarity search
CREATE INDEX ON chunks USING hnsw (embedding vector_cosine_ops);

-- Posts table: drafts/scheduled/published content
CREATE TABLE posts (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  topic TEXT,
  draft TEXT,
  post_text TEXT,
  status TEXT,
  channel_id TEXT,
  scheduled_at TIMESTAMPTZ,
  published_at TIMESTAMPTZ,
  meta JSONB
);

-- Metrics table: views/reactions/comments tracking
CREATE TABLE metrics (
  id SERIAL PRIMARY KEY,
  post_id UUID REFERENCES posts(id),
  views INT,
  reactions INT,
  comments INT,
  collected_at TIMESTAMPTZ DEFAULT now()
);

-- Prompts memory: channel voice/style/taboos/hashtags
CREATE TABLE prompts_memory (
  id SERIAL PRIMARY KEY,
  kind TEXT,
  content TEXT,
  updated_at TIMESTAMPTZ DEFAULT now()
);

-- Indexes for performance
CREATE INDEX idx_sources_url ON sources(url);
CREATE INDEX idx_sources_fetched_at ON sources(fetched_at);
CREATE INDEX idx_chunks_source_id ON chunks(source_id);
CREATE INDEX idx_posts_status ON posts(status);
CREATE INDEX idx_posts_scheduled_at ON posts(scheduled_at);
CREATE INDEX idx_metrics_post_id ON metrics(post_id);
CREATE INDEX idx_prompts_memory_kind ON prompts_memory(kind);