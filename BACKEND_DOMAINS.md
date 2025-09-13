# 🏗 Backend Доменная Архитектура - План Изучения

*Этот файл - ваш roadmap для изучения Rust через разработку ADCO backend*

## 🎯 Цель
Изучить продвинутый Rust через реальный проект: async/await, трейты, обработку ошибок, веб-фреймворки, интеграции с внешними API.

## 📚 Предварительные знания (август 2025)
- ✅ Умные указатели (Rc, Arc, Box, RefCell, Mutex)
- ✅ Error handling (anyhow, Result, контекстуализация)  
- ✅ Макросы (declarative, паттерн matching)
- ✅ Derives и кастомные трейты

## 🔗 Новое что изучим
- 🔄 **async/await** - асинхронное программирование в Rust
- 🌐 **Axum** - современный веб-фреймворк 
- 🗄 **SQLx** - работа с базами данных
- 📡 **HTTP клиенты** - интеграция с внешними API
- 🏛 **Архитектурные паттерны** - Repository, Service Layer
- ⚙️ **Dependency Injection** - управление зависимостями

---

## 🏗 Доменная структура

### 1. 📝 posts (НАЧИНАЕМ ОТСЮДА)
**Уровень сложности**: ⭐⭐ (Легкий)  
**Новые концепты**: Базовые модели, derives, serde

**Что изучим**:
- Создание структур данных для постов
- Serde сериализация/десериализация  
- UUID в качестве ID
- Enums для статусов постов

**Структура**:
```
posts/
├── mod.rs        # Публичный интерфейс домена
├── models.rs     # Post, PostStatus, PostMeta
└── repository.rs # Трейт для работы с БД (пока без реализации)
```

### 2. 🗄 sources  
**Уровень сложности**: ⭐⭐⭐ (Средний)  
**Новые концепты**: HTTP клиенты, внешние API, парсинг данных

**Что изучим**:
- reqwest для HTTP запросов
- Асинхронность (async/await)
- Обработка JSON ответов
- Error handling с контекстом

**Структура**:
```
sources/
├── mod.rs
├── models.rs     # Source, SourceType
├── repository.rs # Сохранение источников
├── fetcher.rs    # HTTP клиенты для внешних API
└── parser.rs     # Парсинг контента
```

### 3. 🧠 content
**Уровень сложности**: ⭐⭐⭐⭐ (Сложный)  
**Новые концепты**: RAG, embeddings, vector operations

**Что изучим**:
- rig.rs для RAG операций
- Работа с векторами и embeddings
- Chunking текста
- Интеграция с pgvector

**Структура**:
```
content/
├── mod.rs
├── models.rs     # Chunk, Embedding
├── repository.rs # Работа с pgvector
├── chunker.rs    # Разбиение текста на части  
├── embedder.rs   # Генерация embeddings
└── retriever.rs  # RAG поиск и ранжирование
```

### 4. 🤖 llm
**Уровень сложности**: ⭐⭐⭐ (Средний)  
**Новые концепты**: Интеграция с LLM API, промпт инженеринг

**Что изучим**:
- rig.rs для Claude/GPT интеграции
- Async HTTP запросы к LLM API
- Обработка streaming ответов
- Rate limiting и retry logic

**Структура**:
```
llm/
├── mod.rs
├── models.rs     # LlmProvider, Prompt, Response  
├── claude.rs     # Claude API клиент
├── gpt.rs        # OpenAI API клиент
└── service.rs    # Координация между агентами
```

### 5. 📡 publishing
**Уровень сложности**: ⭐⭐⭐ (Средний)  
**Новые концепты**: Telegram Bot API, WebHooks

**Что изучим**:
- frankenstein для Telegram API
- Async публикация сообщений
- Error handling для сетевых операций
- Rate limiting для API

**Структура**:
```  
publishing/
├── mod.rs
├── models.rs     # TelegramChannel, Publication
├── telegram.rs   # Telegram Bot API клиент
└── service.rs    # Логика публикации
```

### 6. 📊 analytics  
**Уровень сложности**: ⭐⭐ (Легкий)  
**Новые концепты**: Метрики, агрегация данных

**Что изучим**:
- Структуры для метрик
- Агрегация и вычисления
- Временные ряды

**Структура**:
```
analytics/
├── mod.rs  
├── models.rs     # Metrics, Analytics
├── repository.rs # Сохранение метрик
└── calculator.rs # Вычисление показателей
```

### 7. ⏰ scheduler
**Уровень сложности**: ⭐⭐⭐⭐ (Сложный)  
**Новые концепты**: Cron jobs, координация задач, channels

**Что изучим**:
- tokio-cron-scheduler
- Async coordination между доменами
- Channels для межпотоковой коммуникации
- Graceful shutdown

**Структура**:
```
scheduler/
├── mod.rs
├── jobs.rs       # Определение задач
├── coordinator.rs # Координация выполнения
└── service.rs    # Управление планировщиком
```

---

## 📋 План изучения по этапам

### Этап 1: Основы (posts)
- Создаем базовые модели данных
- Изучаем serde и derives
- Пишем простейшие тесты

### Этап 2: HTTP и async (sources)  
- Первое знакомство с async/await
- HTTP клиенты на reqwest
- Error handling в асинхронном коде

### Этап 3: База данных (posts + sources repositories)
- SQLx и работа с PostgreSQL
- Миграции и схема БД
- Async database operations

### Этап 4: LLM интеграция (llm)
- rig.rs и работа с API
- Промпт инженеринг
- Обработка ответов от Claude/GPT

### Этап 5: RAG система (content)  
- Продвинутая работа с rig.rs
- Vector operations и embeddings
- Поиск и ранжирование

### Этап 6: Публикация (publishing)
- Telegram Bot API
- Async публикация
- Обработка ошибок сети

### Этап 7: Веб API (api layer)
- Axum веб-фреймворк
- REST endpoints  
- Middleware и обработка ошибок

### Этап 8: Планировщик (scheduler)
- Координация всех доменов
- Cron jobs и автоматизация
- Production готовность

---

## 🔧 Общие паттерны

### Repository Pattern
```rust
// Трейт для каждого домена
#[async_trait]  
pub trait PostRepository {
    async fn create(&self, post: CreatePost) -> Result<Post>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>>;
    async fn update(&self, post: Post) -> Result<Post>;
}
```

### Service Layer  
```rust
// Бизнес логика домена
pub struct PostService {
    repository: Arc<dyn PostRepository>,
}

impl PostService {
    pub async fn publish_post(&self, id: Uuid) -> Result<()> {
        // Бизнес логика здесь
    }
}
```

### Error Handling
```rust
// Кастомные ошибки для каждого домена
#[derive(thiserror::Error, Debug)]
pub enum PostError {
    #[error("Post not found: {id}")]
    NotFound { id: Uuid },
    
    #[error("Database error: {0}")]  
    Database(#[from] sqlx::Error),
}
```

---

## 🎓 Ожидаемые результаты обучения

После прохождения всех доменов вы будете знать:
- **Async/await** - полное понимание асинхронного Rust
- **Веб-разработку** - создание REST API на Axum
- **Базы данных** - работа с PostgreSQL через SQLx
- **Внешние интеграции** - HTTP клиенты, API
- **Архитектурные паттерны** - Repository, Service Layer
- **Production готовность** - error handling, логирование, тесты

**Следующий проект**: Вы сможете создать любой backend на Rust самостоятельно!

---

*"Мда, неплохой план. Начнем с простого - models для постов. Готов?"* - Rust Sensei