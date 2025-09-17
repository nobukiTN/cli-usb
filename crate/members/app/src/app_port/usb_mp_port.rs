use std::fs;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use std::path::Path;

use crate::app_error::app_error::AppError;
use domain::domain_model::usb_model::{Mp, Usb, MpUuid, UsbUuid};
#[async_trait]
pub trait SqlConn {
    async fn connect(&self, url: &str) -> Result<Pool<Sqlite>, AppError>;

    async fn migrate(&self, pool: &Pool<Sqlite>) -> Result<(), AppError>;
}
#[async_trait]
pub trait MpRepo {
    async fn save(&self, pool: &Pool<Sqlite>, mp: &Mp) -> Result<MpUuid, AppError>;
    async fn delete(&self, pool: &Pool<Sqlite>, id: MpUuid) -> Result<(), AppError>;
    async fn find_by_id(&self, pool: &Pool<Sqlite>, id: MpUuid) -> Result<Option<Mp>, AppError>;
    async fn update(&self, pool: &Pool<Sqlite>, mp: &Mp) -> Result<(), AppError>;

    async fn find_by_path(&self, pool: &Pool<Sqlite>, path: &Path) -> Result<Option<Mp>, AppError>;
}

#[async_trait]
pub trait UsbRepo {
    async fn save(&self, pool: &Pool<Sqlite>, usb: &Usb) -> Result<UsbUuid, AppError>;
    async fn delete(&self, pool: &Pool<Sqlite>, id: UsbUuid) -> Result<(), AppError>;
    async fn find_by_id(&self, pool: &Pool<Sqlite>, id: UsbUuid) -> Result<Option<Usb>, AppError>;
    async fn update(&self, pool: &Pool<Sqlite>, usb: &Usb) -> Result<(), AppError>;
}
#[async_trait]
pub trait MpPlayer {
    async fn play(&self, mp: &Mp) -> Result<(), AppError>;
}