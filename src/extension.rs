use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// 扩展功能管理器
pub struct ExtensionManager {
    subscribers: Arc<Mutex<HashMap<String, Vec<mpsc::UnboundedSender<String>>>>>,
    logger: Arc<OperationLogger>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        ExtensionManager {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            logger: Arc::new(OperationLogger::new()),
        }
    }

    /// 发布消息到主题
    pub async fn publish(&self, topic: String, message: String) -> anyhow::Result<()> {
        let subscribers = self.subscribers.lock().await;
        if let Some(senders) = subscribers.get(&topic) {
            for sender in senders {
                let _ = sender.send(message.clone());
            }
        }
        Ok(())
    }

    /// 订阅主题
    pub async fn subscribe(&self, topic: String) -> mpsc::UnboundedReceiver<String> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut subscribers = self.subscribers.lock().await;
        subscribers.entry(topic).or_insert_with(Vec::new).push(tx);
        rx
    }

    /// 取消订阅
    pub async fn unsubscribe(&self, topic: &str) {
        let mut subscribers = self.subscribers.lock().await;
        subscribers.remove(topic);
    }
}

/// 操作日志记录器
pub struct OperationLogger {
    log_file: Arc<Mutex<std::fs::File>>,
}

impl OperationLogger {
    pub fn new() -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/operation.log")
            .unwrap();
        OperationLogger {
            log_file: Arc::new(Mutex::new(file)),
        }
    }

    pub async fn log(&self, operation: &str, key: &str, value: Option<&str>) -> anyhow::Result<()> {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(
            "[{}] {} key={} value={}\n",
            timestamp,
            operation,
            key,
            value.unwrap_or("None")
        );
        
        let mut file = self.log_file.lock().await;
        use std::io::Write;
        file.write_all(log_entry.as_bytes())?;
        file.flush()?;
        Ok(())
    }
}