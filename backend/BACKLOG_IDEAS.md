# 🔮 ADCO Backend - Backlog Ideas

*Идеи для будущих улучшений, которые НЕ нужны для MVP, но хорошо бы помнить*

## 📝 Posts Domain Improvements

### PostMeta расширения
Текущая версия (MVP):
```rust
pub struct PostMeta {
    pub title: Option<String>,
    pub description: Option<String>, 
    pub image_url: Option<String>,
}
```

**Будущие дополнения:**

#### 📱 Telegram специфичное:
- `parse_mode: Option<String>` - "Markdown", "HTML", "MarkdownV2"
- `disable_web_page_preview: Option<bool>` - отключить превью ссылок
- `hashtags: Vec<String>` - хэштеги для поста
- `reply_markup: Option<serde_json::Value>` - inline кнопки

#### 🔍 Источники и качество:
- `source_urls: Vec<String>` - откуда взята информация
- `fact_checked: Option<bool>` - прошел ли фактчекинг
- `confidence_score: Option<f32>` - уверенность LLM (0.0-1.0)
- `credibility_rating: Option<u8>` - рейтинг надежности источников (1-10)

#### 📊 Аналитика и метрики:
- `target_audience: Option<String>` - целевая аудитория
- `estimated_reading_time: Option<u32>` - время чтения в секундах
- `keywords: Vec<String>` - ключевые слова для поиска/тегирования
- `topic_category: Option<String>` - "AI", "Science", "Philosophy"
- `complexity_level: Option<String>` - "Beginner", "Intermediate", "Advanced"

#### 🤖 AI Workflow метаданные:
- `generated_by: Option<String>` - "claude-3-sonnet", "gpt-4-turbo"
- `research_query: Option<String>` - исходный поисковый запрос
- `rag_sources_count: Option<u32>` - количество RAG источников
- `generation_timestamp: Option<DateTime<Utc>>` - когда был сгенерирован
- `model_temperature: Option<f32>` - настройки модели
- `prompt_version: Option<String>` - версия промпта

#### 🎨 Контент и стиль:
- `tone: Option<String>` - "Scientific", "Casual", "Analytical"
- `emoji_style: Option<String>` - "Minimal", "Normal", "Rich"
- `content_warnings: Vec<String>` - предупреждения о контенте
- `language_level: Option<String>` - уровень языка

#### ⏰ Планирование и оптимизация:
- `optimal_posting_time: Option<DateTime<Utc>>` - лучшее время для публикации
- `seasonal_relevance: Option<String>` - сезонная актуальность
- `trending_score: Option<f32>` - актуальность темы
- `engagement_prediction: Option<f32>` - прогнозируемый engagement

## 🗄️ Sources Domain Ideas

### Source типы:
- Academic papers (arXiv, PubMed)
- News articles
- Blog posts
- Social media
- Podcasts/videos
- Research datasets

### Дополнительные поля Source:
- `authority_score: f32` - авторитетность источника
- `freshness_score: f32` - свежесть информации
- `bias_rating: Option<String>` - политический уклон
- `language: String` - язык источника
- `content_type: SourceContentType` - тип контента

## 🧠 Content Domain Ideas

### 📝 RAG System Extensions (Приоритет: Высокий)
**Дата добавления**: 2025-09-14

#### Prompt Memory RAG
**Проблема**: Нет переиспользования успешных промптов и стилистических решений.

**Решение**:
```sql
CREATE TABLE prompt_memory_chunks (
  id SERIAL PRIMARY KEY,
  prompt_memory_id INT REFERENCES prompts_memory(id),
  text TEXT NOT NULL,
  embedding VECTOR(1536),
  context_type TEXT, -- 'style', 'taboo', 'hashtag', 'voice'
  success_score REAL, -- оценка успешности
  created_at TIMESTAMPTZ DEFAULT now()
);
```

**Польза**:
- Консистентность стиля канала
- Переиспользование удачных формулировок
- Автоматическое обучение на успешных примерах

#### LLM History RAG
**Проблема**: Теряется качественный контент из Claude research и GPT styling.

**Решение**:
```sql
CREATE TABLE llm_interactions (
  id SERIAL PRIMARY KEY,
  model_type TEXT NOT NULL, -- 'claude_research', 'gpt_styling'
  prompt TEXT NOT NULL,
  response TEXT NOT NULL,
  quality_score REAL, -- на основе метрик поста
  created_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE llm_response_chunks (
  id SERIAL PRIMARY KEY,
  interaction_id INT REFERENCES llm_interactions(id),
  text TEXT NOT NULL,
  embedding VECTOR(1536),
  response_type TEXT, -- 'research_finding', 'style_guideline'
  created_at TIMESTAMPTZ DEFAULT now()
);
```

**Польза**:
- Экономия на повторных LLM запросах
- Накопление экспертизы в домене
- Улучшение качества через историю

### Chunking стратегии:
- Adaptive chunking по смыслу
- Overlap между чанками
- Metadata-aware chunking

### Embedding улучшения:
- Multi-language embeddings
- Domain-specific embeddings
- Hybrid search (keyword + semantic)

## 🤖 LLM Domain Ideas

### 🔄 Избежание повторения тем (Приоритет: Средний)
**Дата добавления**: 2025-09-15

**Проблема**: TopicGenerator может генерировать похожие темы, снижая разнообразие контента.

**Варианты решения**:

1. **Семантический поиск дубликатов**:
   - Векторизировать сгенерированные темы через embeddings
   - При генерации новой темы проверять similarity с последними 30 темами
   - Если similarity > 0.8, генерировать заново (до 5 попыток)

2. **Blacklist в промпте**:
   - Добавлять последние 10 тем в контекст TopicGenerator
   - Инструкция избегать похожих формулировок

3. **Расширение тематик**:
   - Увеличить список базовых тем в topic_generator.rs
   - Добавить: quantum physics, space exploration, bioengineering, consciousness studies

4. **Таблица истории тем**:
   ```sql
   CREATE TABLE generated_topics (
     id SERIAL PRIMARY KEY,
     topic_text TEXT NOT NULL,
     embedding VECTOR(1536),
     generated_at TIMESTAMPTZ DEFAULT now()
   );
   ```

**Рекомендуемый подход**: Начать с пункта 2 (контекст в промпте) как самый простой.

### Multi-agent coordination:
- Researcher agent
- Fact-checker agent
- Style editor agent
- Quality control agent

### Prompt engineering:
- Dynamic prompt templates
- Context-aware prompting
- Chain-of-thought reasoning
- Self-reflection loops

## 📡 Publishing Domain Ideas

### Telegram расширения:
- Story posting через userbot
- Scheduled posts with editing
- Poll creation
- File attachments
- Voice messages

### Multi-platform:
- Twitter/X integration
- LinkedIn posts
- Medium articles
- RSS feed generation

## 📊 Analytics Domain Ideas

### Engagement analytics:
- Reaction analysis
- Comment sentiment
- Share/forward tracking
- Read time estimation

### Content optimization:
- A/B test framework
- Performance prediction
- Optimal timing analysis
- Audience segmentation

## ⏰ Scheduler Domain Ideas

### Advanced scheduling:
- Smart timing based on audience activity
- Seasonal content planning
- Breaking news interruption
- Content queue optimization

### Dependency management:
- Task dependencies
- Rollback capabilities
- Health checks
- Graceful degradation

## 🔧 Infrastructure Ideas

### Performance:
- Redis caching layer
- CDN for images
- Database read replicas
- Connection pooling

### Monitoring:
- Custom metrics
- Alerting system
- Performance profiling
- Error tracking

### Security:
- API rate limiting
- Input validation
- Content filtering
- Audit logging

---

## 📝 Как использовать этот файл:

1. **Не для MVP** - все эти идеи для версий 1.1+
2. **Приоритизация** - отмечать звездочками важные идеи
3. **Ссылки на issues** - создавать GitHub issues для больших фич
4. **Регулярный ревью** - раз в месяц пересматривать актуальность

**Помни**: Лучше простая система которая работает, чем сложная которая не готова! 🎯