# ADCO - Async Dev Content Orchestrator

**Full-Stack Rust** система генерации и публикации постов для Telegram канала с использованием LLM агентов, RAG системы и векторного поиска.

**Обновлено (17.09.2025):** Интегрирован Leptos SSR для современного веб-интерфейса

## Описание

ADCO - интеллектуальный оркестратор контента, который:
- Исследует актуальные темы через Claude Sonnet 4
- Агрегирует источники из HackerNews, arXiv, веб-страниц
- Создает качественные посты через GPT
- Использует RAG для контекстуальной генерации с векторным поиском

## Архитектура системы

### Workspace Structure
```
ADCO/
├── backend/     # Rust API server (Axum + SQLx + RIG)
├── frontend/    # Leptos SSR + WASM hydration
└── shared/      # Common types with conditional compilation
```

### Поток обработки

```mermaid
graph TB
    A[Topic Generator] --> B[Search APIs]
    B --> C[Content Fetcher]
    C --> D[Text Chunker]
    D --> E[Vector Embeddings]
    E --> F[RAG Retrieval]
    F --> G[Researcher Agent]
    G --> H[Stylizer Agent]
```

### Компоненты

#### LLM Агенты (3-stage pipeline)
- **Topic Generator**: Генерирует темы и поисковые запросы
- **Researcher**: Анализирует источники и создает фактчекнутые выжимки
- **Stylizer**: Превращает исследования в посты в стиле Async Dev

#### Sources Management
- **HttpFetcher**: Интеграция с HackerNews API, arXiv API
- **HtmlParser**: Извлечение контента из веб-страниц + PDF
- **ChromeFetcher**: Headless браузер для динамического контента

#### RAG System
- **TextChunker**: Разбивка текста на семантические чанки
- **EmbeddingsRepository**: Векторное хранилище с cosine similarity search
- **PostgreSQL + pgvector**: Высокопроизводительный поиск по векторам

#### Infrastructure
- **Orchestrator**: Координация всего пайплайна
- **Postgres Integration**: Сохранение источников, чанков, постов

## Структура проекта

```
ADCO/
├── backend/           # Rust backend
│   ├── src/
│   │   ├── domain/    # Доменная логика
│   │   │   ├── llm/           # LLM агенты
│   │   │   │   └── agents/    # TopicGenerator, Researcher, Stylizer
│   │   │   ├── sources/       # Получение контента
│   │   │   │   ├── fetcher.rs # HTTP/API клиенты (HackerNews, arXiv)
│   │   │   │   ├── parser.rs  # Парсинг HTML/PDF + headless Chrome
│   │   │   │   └── models.rs  # Source структуры
│   │   │   ├── content/       # RAG система
│   │   │   │   ├── chunker.rs # Разбивка текста
│   │   │   │   └── embeddings_repository.rs # Векторное хранилище
│   │   │   ├── posts/         # Управление постами
│   │   │   ├── infra/         # База данных
│   │   │   └── orchestrator.rs # Главный координатор
│   │   ├── api/       # HTTP handlers (заготовка)
│   │   ├── infra/     # Инфраструктурный слой
│   │   ├── utils/     # Общие утилиты
│   │   └── main.rs    # Точка входа
│   ├── posts/         # Сгенерированные посты
│   ├── Cargo.toml     # Зависимости Rust
│   ├── BACKLOG_IDEAS.md       # Идеи для будущих версий
│   └── RIG_RESEARCH.md        # Исследование LLM библиотек
├── sql/
│   └── 01_init.sql    # Схема PostgreSQL + pgvector
├── ARCH.md            # Детальная архитектура
├── CLAUDE.md          # Инструкции для разработки
└── README.md          # Этот файл
```

## Быстрый старт

### Предварительные требования

```bash
# Rust toolchain + WASM target
rustc 1.88.0+
rustup target add wasm32-unknown-unknown

# Leptos CLI
cargo install cargo-leptos

# PostgreSQL с расширением pgvector
postgresql-15+
pgvector-0.5.0+

# Docker (для развертывания БД)
docker, docker-compose
```

### Настройка окружения

1. **Клонирование репозитория**
```bash
git clone <repository-url>
cd ADCO
```

2. **Настройка базы данных**
```bash
# Запуск PostgreSQL через Docker Compose
docker-compose up -d

# Применение миграций
psql -h localhost -U postgres -d adco -f sql/01_init.sql
```

3. **Переменные окружения**
```bash
# .env файл
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/adco
OPENAI_API_KEY=your_openai_api_key
ANTHROPIC_API_KEY=your_anthropic_api_key
```

4. **Запуск backend**
```bash
cd backend
cargo run
```

5. **Запуск frontend (Leptos SSR)**
```bash
cd frontend
cargo leptos watch
# Frontend доступен на http://127.0.0.1:3000
```

### Development
```bash
cd backend
cargo build
cargo run
```

## Текущее состояние (MVP)

### Реализовано
- [x] **Доменная архитектура** - Четкое разделение по доменам
- [x] **LLM интеграция** - Claude + GPT через Rig.rs библиотеку
- [x] **Source fetching** - HackerNews + arXiv + веб-скрапинг
- [x] **RAG система** - Chunking + embeddings + vector search
- [x] **PostgreSQL интеграция** - Схема с pgvector поддержкой
- [x] **Orchestrator** - Полный пайплайн генерации контента

### Текущие возможности

**Полный цикл генерации контента:**
```bash
# Запуск оркестратора
cargo run

# Процесс:
# 1. Topic Generator генерирует тему и поисковые запросы
# 2. Ищет источники в HackerNews + arXiv по запросу
# 3. Скрапит контент (включая PDF и JS-страницы через Chrome)
# 4. Создает чанки и сохраняет embeddings в PostgreSQL
# 5. Выполняет RAG поиск релевантного контента
# 6. Researcher анализирует и создает выжимку
# 7. Stylizer превращает в готовый пост
# 8. Сохраняет в backend/posts/
```

### В разработке (следующие итерации)
- [ ] **Axum веб-сервер** - HTTP API + Leptos SSR интеграция
- [ ] **Leptos админ панель** - Rust full-stack веб-интерфейс с server functions
- [ ] **Мониторинг pipeline** - Real-time статус оркестратора через админку
- [ ] **Telegram публикация** - Bot API интеграция
- [ ] **Scheduler** - Автоматические публикации по расписанию
- [ ] **Metrics collection** - Аналитика просмотров/реакций

## Технологический стек

### Backend (Rust)
- **Runtime**: `tokio` (async/await)
- **Web Server**: `axum` (HTTP API + Leptos SSR)
- **Database**: `sqlx` (PostgreSQL + compile-time checks)
- **LLM**: `rig-core` + `rig-postgres` (Unified LLM API)
- **Parsing**: `scraper`, `readability`, `pdf-extract`
- **Scraping**: `headless_chrome` для динамического контента
- **HTTP Client**: `reqwest` с TLS поддержкой

### Frontend (Rust + WASM)
- **Framework**: `leptos` (reactive UI с server functions)
- **Build Tool**: `trunk` (live-reload development)
- **Styling**: Планируется TailwindCSS
- **State**: Fine-grained reactivity без virtual DOM

### Database
- **Core**: `PostgreSQL 15+`
- **Vectors**: `pgvector` расширение для semantic search
- **Indexes**: HNSW для быстрого cosine similarity поиска

### External APIs
- **LLM Models**:
  - Claude Sonnet 4 (research + extended context)
  - GPT-4 Turbo (fast styling + cost-effective)
- **Content Sources**:
  - HackerNews Algolia Search API
  - arXiv.org API для научных статей
  - Web scraping через headless Chrome

## База данных

### Схема таблиц

```sql
-- Источники контента
sources (id, url, title, source_type, raw_text, fetched_at)

-- RAG векторное хранилище (rig-postgres compatible)
documents (id, document, embedded_text, embedding[1536])

-- Посты канала
posts (id, topic, draft, post_text, status, scheduled_at, meta)
```

### Индексы производительности
- **HNSW** векторный индекс для `documents.embedding`
- **B-Tree** индексы для часто используемых полей

## LLM Workflow

### Агентная архитектура

1. **Topic Generator Agent**
   ```rust
   // Генерирует релевантную тему для канала
   struct TopicGeneratorResult {
       topic: String,        // "Advances in LLM reasoning"
       search_query: String, // "LLM chain-of-thought reasoning 2024"
   }
   ```

2. **Researcher Agent**
   ```rust
   // Анализирует найденные источники через RAG
   struct ResearcherResult {
       content: String,      // Фактчекнутая выжимка с цитатами
       sources_used: Vec<String>, // URLs использованных источников
   }
   ```

3. **Stylizer Agent**
   ```rust
   // Создает пост в стиле Async Dev
   struct StylizerResult {
       content: String,      // Готовый пост
       hashtags: Vec<String>, // Рекомендуемые хэштеги
   }
   ```

### Prompt Engineering

Каждый агент использует специализированные промпты:
- **Topic Generator**: Актуальность + релевантность для DevOps аудитории
- **Researcher**: Фокус на фактчекинг + цитирование источников
- **Stylizer**: Строгий стиль канала (структура, тон)

## Будущие планы

### Следующие итерации
- **HTTP API** - REST endpoints для управления
- **Telegram Integration** - Bot API интеграция
- **Scheduling** - Автоматическая публикация по расписанию
- **Metrics** - Сбор и анализ engagement
- **Web Admin UI** - Интерфейс управления

Детальный список улучшений в файле [`backend/BACKLOG_IDEAS.md`](backend/BACKLOG_IDEAS.md)

## Безопасность и Лимиты

### Ограничения API
- **Rate limiting** для внешних API
- **Retry logic** с экспоненциальными задержками

### Безопасность данных
- **Environment variables** для API ключей
- **SQL injection** защита через `sqlx` prepared statements
- **Input validation** для всех пользовательских данных

## Разработка

### Принципы кодбазы
- **Rust Sensei Mode**: Обучающий подход к Rust разработке
- **DRY + KISS**: Простота и переиспользование кода
- **Domain-Driven**: Четкое разделение по доменам
- **Test-First**: Unit + integration тесты

### Code Style
```rust
// Структуры с документацией и сериализацией
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub topic: String,
    pub status: PostStatus,
    pub created_at: DateTime<Utc>,
}

// Error handling - всегда Result<T, E>
pub async fn fetch_post(id: Uuid) -> Result<Post, DatabaseError> {
    // Implementation with proper error context
}
```

### Contribution Guidelines
1. **Backend first** - Сначала полная реализация API
2. **No frontend** до готовности backend
3. **Russian communication** - Весь код и комментарии на русском
4. **Learning focus** - Приоритет обучению Rust через практику

## Документация

- **[ARCH.md](ARCH.md)** - Полная техническая архитектура
- **[CLAUDE.md](CLAUDE.md)** - Инструкции для разработки и Rust Sensei режим
- **[backend/RIG_RESEARCH.md](backend/RIG_RESEARCH.md)** - Исследование LLM библиотек
- **[backend/BACKLOG_IDEAS.md](backend/BACKLOG_IDEAS.md)** - Идеи для будущих версий

## Использованные технологии
- **Rust** ecosystem (tokio, sqlx, serde)
- **PostgreSQL** с pgvector расширением
- **Anthropic Claude** для research задач
- **OpenAI GPT** для генерации контента
- **Rig.rs** как unified LLM framework