use std::collections::HashMap;
use std::fs::{self, File};
// 只保留使用的导入
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// 数据文件路径
const DATA_DIR: &str = "data";
const DATA_FILE: &str = "kvstore.json";

/// 键值存储核心结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// 创建一个新的空 KvStore
    pub fn new() -> Self {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// 从文件加载数据
    pub fn load_from_file() -> io::Result<Self> {
        let path = Self::get_data_path();
        
        if !path.exists() {
            // 首次启动，返回空存储
            println!("📭 首次启动，创建空数据库");
            return Ok(KvStore::new());
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        
        match serde_json::from_reader(reader) {
            Ok(store) => {
                println!("📂 成功加载数据文件: {:?}", path);
                Ok(store)
            }
            Err(e) => {
                // 数据文件损坏，给出明确错误
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("数据文件损坏: {}", e),
                ))
            }
        }
    }

    /// 保存数据到文件
    pub fn save_to_file(&self) -> io::Result<()> {
        let path = Self::get_data_path();
        
        // 确保目录存在
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
                println!("📁 创建数据目录: {:?}", parent);
            }
        }

        let file = File::create(&path)?;
        let writer = BufWriter::new(file);
        
        serde_json::to_writer_pretty(writer, &self)?;
        println!("💾 数据已保存到: {:?}", path);
        
        Ok(())
    }

    /// 获取数据文件路径
    fn get_data_path() -> PathBuf {
        let mut path = PathBuf::from(DATA_DIR);
        path.push(DATA_FILE);
        path
    }

    /// 检查数据文件是否存在
    pub fn data_file_exists() -> bool {
        Self::get_data_path().exists()
    }

    /// 删除数据文件（用于测试）
    #[cfg(test)]
    pub fn delete_data_file() -> io::Result<()> {
        let path = Self::get_data_path();
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// 插入或更新一个键值对
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// 获取指定键的值
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// 删除指定键值对
    pub fn remove(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    /// 列出所有键
    pub fn list_keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    /// 获取当前存储中的键值对数量
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查存储是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 清空所有数据
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_new_store_is_empty() {
        let store = KvStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = KvStore::new();
        store.set("name".to_string(), "Rust".to_string());
        assert_eq!(store.get("name"), Some(&"Rust".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = KvStore::new();
        assert_eq!(store.get("nonexistent"), None);
    }

    #[test]
    fn test_set_overwrites_existing_key() {
        let mut store = KvStore::new();
        store.set("key".to_string(), "value1".to_string());
        assert_eq!(store.get("key"), Some(&"value1".to_string()));
        store.set("key".to_string(), "value2".to_string());
        assert_eq!(store.get("key"), Some(&"value2".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_remove_existing_key() {
        let mut store = KvStore::new();
        store.set("key".to_string(), "value".to_string());
        assert!(store.remove("key"));
        assert_eq!(store.get("key"), None);
        assert!(store.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut store = KvStore::new();
        assert!(!store.remove("nonexistent"));
    }

    #[test]
    fn test_list_keys() {
        let mut store = KvStore::new();
        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());
        store.set("c".to_string(), "3".to_string());
        let mut keys = store.list_keys();
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_clear() {
        let mut store = KvStore::new();
        store.set("key1".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value2".to_string());
        assert_eq!(store.len(), 2);
        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_default_implementation() {
        let store = KvStore::default();
        assert!(store.is_empty());
    }

    // 持久化测试
    #[test]
    fn test_save_and_load() -> io::Result<()> {
        // 使用临时目录避免干扰实际数据
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path().join("test.json");
        
        // 创建一个存储并保存
        let mut store1 = KvStore::new();
        store1.set("key1".to_string(), "value1".to_string());
        store1.set("key2".to_string(), "value2".to_string());
        
        // 手动保存到临时文件
        let file = File::create(&temp_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &store1)?;
        
        // 从临时文件加载
        let file = File::open(&temp_path)?;
        let reader = BufReader::new(file);
        let store2: KvStore = serde_json::from_reader(reader)?;
        
        // 验证数据一致
        assert_eq!(store2.get("key1"), Some(&"value1".to_string()));
        assert_eq!(store2.get("key2"), Some(&"value2".to_string()));
        assert_eq!(store2.len(), 2);
        
        Ok(())
    }

    #[test]
    fn test_load_nonexistent_file() -> io::Result<()> {
        // 先删除可能存在的文件
        let _ = KvStore::delete_data_file();
        
        // 加载应该返回空存储
        let store = KvStore::load_from_file()?;
        assert!(store.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_comprehensive_workflow() {
        let mut store = KvStore::new();
        store.set("user1".to_string(), "Alice".to_string());
        store.set("user2".to_string(), "Bob".to_string());
        store.set("user3".to_string(), "Charlie".to_string());
        assert_eq!(store.len(), 3);
        
        assert_eq!(store.get("user1"), Some(&"Alice".to_string()));
        assert_eq!(store.get("user2"), Some(&"Bob".to_string()));
        assert_eq!(store.get("user3"), Some(&"Charlie".to_string()));
        assert_eq!(store.get("user4"), None);
        
        store.set("user1".to_string(), "Alice Updated".to_string());
        assert_eq!(store.get("user1"), Some(&"Alice Updated".to_string()));
        assert_eq!(store.len(), 3);
        
        assert!(store.remove("user2"));
        assert_eq!(store.get("user2"), None);
        assert_eq!(store.len(), 2);
        assert!(!store.remove("user2"));
        
        let mut keys = store.list_keys();
        keys.sort();
        assert_eq!(keys, vec!["user1", "user3"]);
        
        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }
}