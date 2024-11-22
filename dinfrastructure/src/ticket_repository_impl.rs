use crate::TicketCollection;
use color_eyre::Result;
use ddomain::domain_errors::DomainError;
use ddomain::entites::ticket::Ticket;
use ddomain::repositories::ticket_repository::TicketRepository;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};
use toml;

pub struct TicketRepositoryImpl {
    file_path: String,
    ticket_cache: Arc<RwLock<Vec<Ticket>>>, // チケットキャッシュ
}

impl TicketRepositoryImpl {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            ticket_cache: Arc::new(RwLock::new(Vec::new())), // 空のキャッシュで初期化
        }
    }

    fn deserial_toml_file<T>(&self) -> Result<T, DomainError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let file_str = fs::read_to_string(&self.file_path).map_err(DomainError::FileRead)?;
        if file_str.trim().is_empty() {
            Err(DomainError::EmptyFile)
        } else {
            toml::from_str(&file_str).map_err(DomainError::TomlParse)
        }
    }

    fn load_tickets_from_file(&self) -> Result<Vec<Ticket>, DomainError> {
        let ticket_collection: TicketCollection = self.deserial_toml_file::<TicketCollection>()?;
        Ok(ticket_collection.ticket_data)
    }
}

impl TicketRepository for TicketRepositoryImpl {
    fn fetch_tickets(&self) -> Result<Vec<Ticket>, DomainError> {
        // キャッシュが空でないかチェック
        let cache = self.ticket_cache.read().unwrap();
        if cache.is_empty() {
            // キャッシュが空の場合、ファイルから読み込んでキャッシュに保存
            drop(cache);
            let tickets = self.load_tickets_from_file()?;
            let mut cache = self.ticket_cache.write().unwrap();
            *cache = tickets.clone(); // チケットをキャッシュに保存
            Ok(tickets)
        } else {
            // キャッシュが既にある場合、それを返す
            Ok(cache.clone())
        }
    }
    fn ensure_file_exists_with_template(&self) -> Result<()> {
        let path = Path::new(&self.file_path);

        // ファイルが存在しない場合にエラーを返す
        if !path.exists() {
            return Err(DomainError::FileNotFound(self.file_path.clone()).into());
        }

        Ok(())
    }
}
