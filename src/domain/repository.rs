use anyhow::Result;
use crate::domain::App;

pub trait Persistence: Send + Sync {
    fn save(&self, app: &App) -> Result<()>;
    fn load(&self) -> Result<Option<App>>;
    fn delete_session(&self) -> Result<()>;
}
