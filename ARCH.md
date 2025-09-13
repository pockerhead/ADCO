# 📑 Async Dev Content Orchestrator — Design Document

## 🎯 Цель
Создать автономную систему генерации и публикации постов в Telegram-канал **Async Dev**, которая:
- Использует LLM (Claude + GPT) для ресёрча и стилизации.
- Хранит и обогащает материалы через RAG (chunking → embeddings → retrieval → re-rank).
- Публикует посты и сторис в Telegram (бот + userbot).
- Управляется через веб-админку (SPA).
- Учитcя на метриках (реакции/просмотры) и корректирует стиль/тайминг.

---

## 🏗 Архитектура

### 1. Оркестратор (Rust)
- **Axum** + **Tokio**: HTTP API, scheduler jobs.
- **Scheduler**: `tokio-cron-scheduler` для периодических задач.
- **БД**: Postgres + `pgvector`.

### 2. Хранилище
- Таблицы:
  - `sources`: сырые документы (поиск, RSS, статьи).
  - `chunks`: чанки + эмбеддинги + мета.
  - `posts`: посты (draft/scheduled/published).
  - `metrics`: просмотры/реакции.
  - `prompts_memory`: голос канала (стиль, табу, хэштеги).

### 3. LLM-Агенты
- **Claude (Anthropic $100 plan)**:
  - Роль: *ресёрчер/редактор*.
  - Задачи: генерировать темы, писать поисковые запросы, делать выжимки из RAG.
  - Преимущества: большой контекст (200k–1M токенов), extended thinking, web-fetch.

- **GPT (OpenAI Plus $20)**:
  - Роль: *стилист/копирайтер*.
  - Задачи: превращать выжимки в Async Dev-посты (Markdown, эмодзи, TL;DR).
  - Преимущества: дешёвый, быстрый, отлично следует инструкциям.

### 4. Поиск
- Внешние API: Bing, SerpAPI, arXiv, HackerNews.
- Claude формирует запросы → сервис ищет → результаты складываются в `sources`.

### 5. RAG
- **Chunking**: 300–500 токенов, split по абзацам.
- **Embeddings**:
  - быстрый старт: OpenAI `text-embedding-3-small`.
  - продвинуто: локальные модели (`bge-large`, `gte-large`) через ONNX/sidecar.
- **Storage**: pgvector (cosine similarity).
- **Retrieval**: top-K nearest neighbors.
- **Re-ranking**:
  - baseline: cosine top-K,
  - advanced: cross-encoder (`bge-reranker-large`),
  - опционально: LLM-judge (Claude).

### 6. Публикация
- **Telegram Bot API** → посты сразу.
- **Telethon userbot** → сторис, отложенные публикации.
- Логи статусов в `posts`.

### 7. Админка
- **Nuxt 3 + Vue 3** - SSR/SSG фреймворк с TypeScript.
- **Shared типы** - общие TypeScript интерфейсы между фронтом и беком.
- **REST API** - простые JSON endpoints без GraphQL пока.
- Функции:
  - список постов (draft/scheduled/published),
  - редактирование, ручная публикация,
  - просмотр логов/метрик.

### 8. Метрики и Feedback
- Сбор `views/reactions/comments` через Telethon.
- Сохраняем в `metrics`.
- Аналитика → хинты для Claude (что заходит, длина, тайминг, темы).
- A/B тесты: 2 варианта поста → сравнение реакции.

---

## 📊 Поток данных

1. **Claude** генерирует тему + поисковые запросы.
2. **Search API** → источники.
3. **Chunking + embeddings** → векторное хранилище.
4. **Retrieval + re-rank** → релевантные чанки.
5. **Claude** делает фактчекнутую выжимку.
6. **GPT** стилизует → Async Dev пост.
7. **Scheduler** публикует (Bot API / Telethon).
8. **Metrics loop** анализирует реакции → обновляет стиль/тайминг.

---

## 📂 SQL Schema (минимум)

```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  url TEXT,
  title TEXT,
  source_type TEXT,
  fetched_at TIMESTAMPTZ,
  raw_text TEXT
);

CREATE TABLE chunks (
  id SERIAL PRIMARY KEY,
  source_id INT REFERENCES sources(id),
  text TEXT NOT NULL,
  embedding VECTOR(1536),
  meta JSONB,
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX ON chunks USING hnsw (embedding vector_cosine_ops);

CREATE TABLE posts (
  id UUID PRIMARY KEY,
  topic TEXT,
  draft TEXT,
  post_text TEXT,
  status TEXT,
  channel_id TEXT,
  scheduled_at TIMESTAMPTZ,
  published_at TIMESTAMPTZ,
  meta JSONB
);

CREATE TABLE metrics (
  id SERIAL PRIMARY KEY,
  post_id UUID REFERENCES posts(id),
  views INT,
  reactions INT,
  comments INT,
  collected_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE prompts_memory (
  id SERIAL PRIMARY KEY,
  kind TEXT,
  content TEXT,
  updated_at TIMESTAMPTZ DEFAULT now()
);
```

---

## 🔧 Технологии

- **Backend**: Rust (axum, sqlx, utoipa, reqwest, tokio-cron-scheduler).
- **DB**: Postgres + pgvector.
- **LLM Clients**: Claude API, OpenAI API.
- **Search**: Bing/SerpAPI.
- **Publisher**: Telegram Bot API + Telethon sidecar.
- **Admin UI**: Nuxt 3 + Vue 3 + TypeScript.
- **Shared типы**: TypeScript интерфейсы между фронтом и беком.
- **Observability**: tracing + Prometheus + Grafana.
- **Security**: JWT + RBAC, secret manager для API keys.

---

## 🚀 MVP → V1

- **MVP (1–2 недели)**:
  - Claude генерит темы + запросы.
  - Search → ingestion.
  - Embeddings (OpenAI API).
  - Simple RAG (top-K cosine).
  - GPT стилизует.
  - Bot API публикует.
  - Базовая админка на Nuxt+Vue.

- **V0.9**:
  - Scheduler + scheduled_at.
  - Telethon для сторис/отложки.
  - Метрики.
  - Prompts memory.

- **V1.0**:
  - Re-rank (cross-encoder).
  - A/B тесты.
  - Аналитика → корректировка стиля/тайминга.

---

## 🔮 Будущие доработки (после MVP)

### Надежность и масштабируемость
- **Message broker** (Redis/RabbitMQ) для очереди задач
- **Circuit breaker pattern** для внешних API
- **Rate limiting** с backoff для LLM API
- **Retry logic** с экспоненциальными задержками

### Качество контента
- **Content scoring** - фильтрация низкокачественных источников
- **Третий агент-модератор** для проверки на токсичность
- **Quality control** - автоматическая проверка перед публикацией
- **Fallback механизмы** при отсутствии источников

### База данных
- **Таблица search_queries** для аналитики поисковых запросов
- **Связь post_sources** для отслеживания использованных источников
- **Расширенные метрики** с детализацией реакций
- **Партиционирование** для больших объемов данных

### Аналитика
- **Детальная аналитика** engagement по времени/темам
- **A/B тесты** с автоматическим выбором лучших вариантов
- **Predictive analytics** для оптимального тайминга
- **Content performance ML** для рекомендаций тем

### Интеграции
- **Real-time обновления** через WebSocket/SSE
- **Кеширование LLM ответов** для экономии
- **Локальные embeddings** через ONNX/sidecar
- **Multiple channels** support

---

## ⚡ Итог
Async Dev Content Orchestrator = **контент-конвейер**:
- Claude = мозг-исследователь.
- GPT = перо-стилист.
- Rust-сервер = сердце (шедулер + API).
- pgvector = память.
- Telegram = голос. 
