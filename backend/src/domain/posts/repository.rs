use super::models::{Post};

pub trait PostRepository {
    fn create_post(&self, post: &Post) -> Result<(), String>;
    fn get_post_by_id(&self, id: &str) -> Result<Option<Post>, String>;
    fn update_post(&self, post: &Post) -> Result<(), String>;
    fn delete_post(&self, id: &str) -> Result<(), String>;
}