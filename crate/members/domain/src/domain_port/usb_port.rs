use uuid::Uuid;
use std::path::{Path, PathBuf};
use crate::domain_error::usb_mp_errors::UsbMpError;

#[derive(Debug, Clone)]
pub enum UsbAction {
    Add,
    Remove,
    Change,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct UsbDeviceInfo {
    pub devnode: Option<PathBuf>,  
    pub syspath: PathBuf,         
    pub subsystem: Option<String>,
    pub devtype: Option<String>, 
    pub serial: Option<String>,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub vid: Option<String>,
    pub pid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UsbEvent {
    pub info: UsbDeviceInfo,
    pub action: UsbAction,
}


pub trait GeneratorUuid: Send + Sync {
    fn generate_usb_id(&self) -> crate::domain_model::usb_model::UsbUuid;
    fn generate_mp_id(&self) -> crate::domain_model::usb_model::MpUuid;
}


pub trait Clock: Send + Sync {
    fn now(&self) -> Result<time::OffsetDateTime,UsbMpError>;
}
pub trait MountPath: Send + Sync {
    fn resolve_actual_mount_path(&self, path: impl AsRef<Path>) -> Result<PathBuf, UsbMpError>;
}


pub trait MkDir: Send + Sync {
    fn ensure_save_dir_parent(&self) -> Result<PathBuf, UsbMpError>;
    fn generate_title_id(&self,)-> Result<Uuid,UsbMpError>;
    fn create_dir_title(&self,base:PathBuf,title_uuid:Uuid)->Result<PathBuf,UsbMpError>;
    fn create_img_path(&self,save_dir:PathBuf, img_path: impl AsRef<Path>) -> Result<PathBuf, UsbMpError>;
    fn create_video_path(&self,save_dir:PathBuf, video_path: impl AsRef<Path>) -> Result<PathBuf, UsbMpError>;
}


pub trait UsbPort: Send + Sync {
    
    fn enumerate(&self) -> Result<Vec<UsbDeviceInfo>, UsbMpError>;

    fn watch(&self) -> Result<Box<dyn Iterator<Item = Result<UsbEvent, UsbMpError>> + Send>, UsbMpError>;
}

