# 📑 ADCO - Async Dev Content Orchestrator

> Автономная система генерации и публикации постов для Telegram канала **Async Dev** с использованием LLM агентов, RAG системы и векторного поиска.

## 🎯 Краткое описание

ADCO - это интеллектуальный оркестратор контента, который:
- **Исследует актуальные темы** через Claude Sonnet 4 (расширенный контекст + web-поиск)
- **Агрегирует источники** из HackerNews, arXiv, PDF, веб-страниц
- **Создает качественные посты** через GPT (быстрый + дешевый стилист)
- **Использует RAG** для контекстуальной генерации с векторным поиском
- **Публикует автономно** в Telegram через Bot API и userbot

## 🏗 Архитектура системы

### 🔄 Поток обработки

```mermaid
graph TB
    A[Topic Generator] --> B[Search APIs]
    B --> C[Content Fetcher]
    C --> D[Text Chunker]
    D --> E[Vector Embeddings]
    E --> F[RAG Retrieval]
    F --> G[Researcher Agent]
    G --> H[Stylizer Agent]
    H --> I[Telegram Publisher]
```

### 🧩 Компоненты

#### 🤖 **LLM Агенты** (3-stage pipeline)
- **Topic Generator**: Генерирует темы и поисковые запросы
- **Researcher**: Анализирует источники и создает фактчекнутые выжимки
- **Stylizer**: Превращает исследования в посты в стиле Async Dev

#### 🔍 **Sources Management**
- **HttpFetcher**: Интеграция с HackerNews API, arXiv API
- **HtmlParser**: Извлечение контента из веб-страниц + PDF
- **ChromeFetcher**: Headless браузер для динамического контента

#### 🧠 **RAG System**
- **TextChunker**: Разбивка текста на семантические чанки
- **EmbeddingsRepository**: Векторное хранилище с cosine similarity search
- **PostgreSQL + pgvector**: Высокопроизводительный поиск по векторам

#### 🔧 **Infrastructure**
- **Orchestrator**: Координация всего пайплайна
- **Postgres Integration**: Сохранение источников, чанков, постов
- **Error Handling**: Graceful degradation и retry logic

## 📦 Структура проекта

```
ADCO/
├── backend/           # 🦀 Rust API backend
│   ├── src/
│   │   ├── domain/    # Доменная логика
│   │   │   ├── llm/           # LLM агенты
│   │   │   │   ├── agents/    # TopicGenerator, Researcher, Stylizer
│   │   │   │   └── mod.rs     # LLM интеграции
│   │   │   ├── sources/       # Получение контента
│   │   │   │   ├── fetcher.rs # HTTP/API клиенты
│   │   │   │   ├── parser.rs  # Парсинг HTML/PDF
│   │   │   │   └── models.rs  # Source структуры
│   │   │   ├── content/       # RAG система
│   │   │   │   ├── chunker.rs # Разбивка текста
│   │   │   │   └── embeddings_repository.rs # Векторное хранилище
│   │   │   ├── posts/         # Управление постами
│   │   │   ├── infra/         # База данных
│   │   │   └── orchestrator.rs # Главный координатор
│   │   └── main.rs    # Точка входа
│   ├── Cargo.toml     # Зависимости Rust
│   ├── BACKLOG_IDEAS.md       # Идеи для будущих версий
│   └── RIG_RESEARCH.md        # Исследование LLM библиотек
├── sql/
│   └── 01_init.sql    # Схема PostgreSQL + pgvector
├── ARCH.md            # 📋 Детальная архитектура
├── CLAUDE.md          # 🤖 Инструкции для разработки
└── README.md          # 📖 Этот файл
```

## 🚀 Быстрый старт

### Предварительные требования

```bash
# Rust toolchain
rustc 1.88.0+

# PostgreSQL с расширением pgvector
postgresql-15+
pgvector-0.5.0+

# Docker (для быстрого развертывания БД)
docker, docker-compose
```

### Настройка окружения

1. **Клонирование репозитория**
```bash
git clone https://github.com/yourusername/ADCO
cd ADCO
```

2. **Настройка базы данных**
```bash
# Запуск PostgreSQL через Docker
docker run -d --name adco-postgres \
  -e POSTGRES_PASSWORD=your_password \
  -e POSTGRES_DB=adco \
  -p 5432:5432 \
  pgvector/pgvector:pg15

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
cargo build --release
cargo run
```

## 🧪 Текущее состояние (MVP)

### ✅ Реализовано
- [x] **Доменная архитектура** - Четкое разделение по доменам
- [x] **LLM интеграция** - Claude + GPT через Rig.rs библиотеку
- [x] **Source fetching** - HackerNews + arXiv + веб-скрапинг
- [x] **RAG система** - Chunking + embeddings + vector search
- [x] **PostgreSQL интеграция** - Схема с pgvector поддержкой
- [x] **Orchestrator** - Полный пайплайн от темы до поста

### 🔄 Текущие возможности

**Полный цикл генерации контента:**
```bash
# Запуск оркестратора
cargo run

# Процесс:
# 1. Генерирует тему (например: "AI reasoning capabilities")
# 2. Создает поисковые запросы
# 3. Ищет источники в HackerNews + arXiv
# 4. Скрапит контент, создает чанки
# 5. Сохраняет embeddings в PostgreSQL
# 6. Выполняет RAG поиск релевантного контента
# 7. Researcher анализирует и создает выжимку
# 8. Stylizer превращает в Async Dev пост
```

### 🚧 В разработке (следующие итерации)
- [ ] **HTTP API** - REST endpoints для админки
- [ ] **Telegram публикация** - Bot API + userbot интеграция
- [ ] **Scheduler** - Автоматические публикации по расписанию
- [ ] **Metrics collection** - Аналитика просмотров/реакций
- [ ] **Web админка** - Nuxt.js интерфейс управления

## 🔧 Технологический стек

### Backend (Rust)
- **🦀 Runtime**: `tokio` (async/await)
- **🌐 HTTP**: `axum` + `tower-http`
- **🗄️ Database**: `sqlx` (PostgreSQL + compile-time checks)
- **🤖 LLM**: `rig-core` + `rig-postgres` (Unified LLM API)
- **📄 Parsing**: `scraper`, `readability`, `pdf-extract`
- **🕷️ Scraping**: `headless_chrome` для динамического контента
- **⏰ Scheduling**: `tokio-cron-scheduler`
- **🔍 HTTP Client**: `reqwest` с TLS поддержкой

### Database
- **📊 Core**: `PostgreSQL 15+`
- **🧠 Vectors**: `pgvector` расширение для semantic search
- **📈 Indexes**: HNSW для быстрого cosine similarity поиска

### External APIs
- **🧠 LLM Models**:
  - Claude Sonnet 4 (research + extended context)
  - GPT-4 Turbo (fast styling + cost-effective)
- **📰 Content Sources**:
  - HackerNews Algolia Search API
  - arXiv.org API для научных статей
  - Web scraping через headless Chrome

## 📊 База данных

### Схема таблиц

```sql
-- Источники контента
sources (id, url, title, source_type, raw_text, fetched_at)

-- RAG векторное хранилище (rig-postgres compatible)
documents (id, document, embedded_text, embedding[1536])

-- Посты канала
posts (id, topic, draft, post_text, status, scheduled_at, meta)

-- Метрики engagement
metrics (post_id, views, reactions, comments, collected_at)

-- Память промптов (стиль канала)
prompts_memory (kind, content, updated_at)
```

### Индексы производительности
- **HNSW** векторный индекс для `documents.embedding`
- **B-Tree** индексы для часто используемых полей
- **JSONB** индексы для метаданных

## 🤖 LLM Workflow

### Агентная архитектура

1. **Topic Generator Agent**
   ```rust
   // Генерирует релевантную тему для канала Async Dev
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
       content: String,      // Готовый Telegram пост с emoji + структурой
       hashtags: Vec<String>, // Рекомендуемые хэштеги
   }
   ```

### Prompt Engineering

Каждый агент использует специализированные промпты:
- **Researcher**: Фокус на фактчекинг + цитирование источников
- **Stylizer**: Строгий стиль канала (emoji, структура, тон)
- **Topic Generator**: Актуальность + релевантность для DevOps аудитории

## 🎯 Будущие планы (Roadmap)

### Version 0.9 (Near Future)
- **HTTP API** - REST endpoints для управления
- **Telegram Integration** - Bot API + Telethon userbot
- **Scheduling** - Автоматическая публикация по расписанию
- **Metrics** - Сбор и анализ engagement

### Version 1.0 (Production Ready)
- **Web Admin UI** - Nuxt.js админка с TypeScript
- **Advanced RAG** - Re-ranking + cross-encoder модели
- **A/B Testing** - Автоматический выбор лучших вариантов
- **Multi-channel** - Поддержка нескольких каналов

### Version 1.1+ (Advanced Features)
- **Local LLM** - Ollama интеграция для независимости
- **Real-time Updates** - WebSocket уведомления
- **Content Scoring** - ML модели для оценки качества
- **Circuit Breakers** - Отказоустойчивость внешних API

Детальный список улучшений в файле [`backend/BACKLOG_IDEAS.md`](backend/BACKLOG_IDEAS.md)

## 🔒 Безопасность и Лимиты

### Ограничения API
- **Rate limiting** для внешних API
- **Retry logic** с экспоненциальными задержками
- **Circuit breaker** паттерн для защиты от сбоев

### Безопасность данных
- **Environment variables** для API ключей
- **SQL injection** защита через `sqlx` prepared statements
- **Input validation** для всех пользовательских данных

## 🤝 Разработка

### Принципы кодбазы
- **🦀 Rust Sensei Mode**: Обучающий подход к Rust разработке
- **📐 DRY + KISS**: Простота и переиспользование кода
- **🏗️ Domain-Driven**: Четкое разделение по доменам
- **🧪 Test-First**: Unit + integration тесты
- **📚 Documentation**: Inline docs для всех публичных API

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

## 📚 Документация

- **[ARCH.md](ARCH.md)** - Полная техническая архитектура
- **[CLAUDE.md](CLAUDE.md)** - Инструкции для разработки и Rust Sensei режим
- **[backend/RIG_RESEARCH.md](backend/RIG_RESEARCH.md)** - Исследование LLM библиотек
- **[backend/BACKLOG_IDEAS.md](backend/BACKLOG_IDEAS.md)** - Идеи для будущих версий

## 📞 Контакты и поддержка

Проект разрабатывается в рамках изучения Rust и современных LLM технологий.

### Использованные технологии
- **Rust** ecosystem (tokio, axum, sqlx, serde)
- **PostgreSQL** с pgvector расширением
- **Anthropic Claude** для research задач
- **OpenAI GPT** для генерации контента
- **Rig.rs** как unified LLM framework

---

**ADCO** - Bringing AI-powered content orchestration to the next level! 🚀🤖

*"Автоматизация контента через интеллектуальную оркестрацию"*