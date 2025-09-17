use time::{OffsetDateTime};
use uuid::{Uuid};
use std::path::{Path,PathBuf};
use ::udev::{Device,Enumerator,MonitorBuilder,MonitorSocket,Context};
use std::fs;
use domain::{domain_error::usb_mp_errors::UsbMpError, domain_port::usb_port::{Clock, MkDir}};

pub struct SysClock;

impl Clock for SysClock {
    fn now(&self) -> Result<OffsetDateTime, UsbMpError> {

        let dt = OffsetDateTime::now_local()
            .map_err(|e| UsbMpError::AnyError("err".to_string()))?;
        Ok(dt)
    }
}

pub struct CreatePath {
    path: PathBuf,
}

impl CreatePath {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl MkDir for CreatePath {
  
    fn ensure_save_dir_parent(&self) -> Result<PathBuf, UsbMpError> {
        fs::create_dir_all(&self.path)
            .map_err(|e| UsbMpError::AnyError("err".to_string()))?;
        Ok(self.path.clone())
    }

    fn generate_title_id(&self) -> Result<Uuid, UsbMpError> {
        Ok(Uuid::new_v4())
    }


    fn create_dir_title(&self, base: PathBuf, title_uuid: Uuid) -> Result<PathBuf, UsbMpError> {
        let folder_name = title_uuid.to_string();
        let dir = base.join(folder_name);
        fs::create_dir_all(&dir)
            .map_err(|e| UsbMpError::AnyError("err".to_string()))?;
        Ok(dir)
    }

    fn create_img_path(
        &self,
        save_dir: PathBuf,
        img_path: impl AsRef<Path>,
    ) -> Result<PathBuf, UsbMpError> {
        Ok(save_dir.join(img_path))
    }

    fn create_video_path(
        &self,
        save_dir: PathBuf,
        video_path: impl AsRef<Path>,
    ) -> Result<PathBuf, UsbMpError> {
        Ok(save_dir.join(video_path))
    }
}