use std::future::Future;

use tracing::Level;

use crate::Value;

use super::{Node, TypedNode};

pub struct Log {
    pub level: Level,
}

impl Default for Log {
    fn default() -> Self {
        Self { level: Level::INFO }
    }
}

impl Log {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

impl TypedNode<String, String> for Log {}

impl Node for Log {
    fn run(&mut self, input: Value) -> Box<dyn Future<Output = Value> + Unpin> {
        let level = self.level;

        let message = match input {
            Value::String(message) => message,
            _ => panic!("Expected String"),
        };

        Box::new(Box::pin(async move {
            match level {
                Level::TRACE => tracing::trace!("{}", message),
                Level::DEBUG => tracing::debug!("{}", message),
                Level::INFO => tracing::info!("{}", message),
                Level::WARN => tracing::warn!("{}", message),
                Level::ERROR => tracing::error!("{}", message),
            };

            Value::String(message)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_log() {
        let mut log = Log::default();

        let value = "Hello, world!".to_string();
        let out = log.run_typed(value.clone()).await.unwrap();
        assert_eq!(out, value);

        let value = Value::String("Hello, world!".to_string());
        let out = log.run(value.clone()).await;
        assert_eq!(out, value);
    }
}