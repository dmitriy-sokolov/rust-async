use tokio::sync::Mutex;

#[derive(Default)]
pub struct Temperature(Mutex<f32>);

impl Temperature {
    pub async fn get(&self) -> f32 {
        *self.0.lock().await
    }

    pub async fn set(&self, val: f32) {
        *self.0.lock().await = val
    }
}
