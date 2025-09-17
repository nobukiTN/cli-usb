use uuid::{Uuid};
use std::str::FromStr;
use time::{OffsetDateTime};
use std::path::{PathBuf,Path};
use crate::domain_error::usb_mp_errors::UsbMpError;
pub struct UsbUuid(Uuid);
pub struct MpUuid(Uuid);
pub struct Usb{
    serial:String,
    id:UsbUuid,
    get_at: OffsetDateTime,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoExt {
    Mp4, M4v, Webm, Ogv, Mpeg, Mpg, Mpe, Avi, Mov,
}

impl FromStr for VideoExt {
    type Err = UsbMpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "mp4" => Self::Mp4, "m4v" => Self::M4v, "webm" => Self::Webm,
            "ogv" => Self::Ogv, "mpeg" => Self::Mpeg, "mpg" => Self::Mpg,
            "mpe" => Self::Mpe, "avi" => Self::Avi, "mov" => Self::Mov,
            other => return Err(UsbMpError::AnyError("err".to_string())),
        })
    }
}

impl TryFrom<&Path> for VideoExt {
    type Error = UsbMpError;
    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        let ext = p.extension().ok_or(UsbMpError::AnyError("err".to_string()))?
            .to_str().ok_or(UsbMpError::AnyError("err".to_string()))?;
        ext.parse()
    }
}

impl VideoExt {
    pub fn from_path(path: &Path) -> Option<Self> {
        Self::try_from(path).ok()
    }
    pub fn is_video(path: &Path) -> bool {
        Self::from_path(path).is_some()
    }
}

pub struct Mp {
    id: MpUuid,
    get_at:OffsetDateTime,
    video_ext: VideoExt,
    video_path: PathBuf,
    img_path: PathBuf,
    save_dir_path: PathBuf,
}
impl Mp {

    pub fn new(
        id: MpUuid,
        get_at:OffsetDateTime,
        video_path: impl AsRef<Path>,
        img_path: impl AsRef<Path>,
        save_dir_path: impl AsRef<Path>,
    ) -> Result<Self, UsbMpError> {
        let video_path_ref = video_path.as_ref();

        let ext = video_path_ref
            .extension().ok_or(UsbMpError::AnyError("err".to_string()))?
            .to_str().ok_or(UsbMpError::AnyError("err".to_string()))?;

              let video_ext = VideoExt::from_str(ext)?; 

        Ok(Self {
            id,
            get_at,
            video_ext,
            video_path: video_path_ref.to_path_buf(),
            img_path: img_path.as_ref().to_path_buf(),
            save_dir_path: save_dir_path.as_ref().to_path_buf(),
        })
    }

    pub fn video_ext(&self) -> VideoExt { self.video_ext }
    pub fn when_get(&self) -> OffsetDateTime{self.get_at}
    pub fn video_path(&self) -> &Path { &self.video_path }
    pub fn img_path(&self) -> &Path { &self.img_path }
    pub fn save_dir_path(&self) -> &Path { &self.save_dir_path }
}

impl Usb {
    pub fn new(usb_id: UsbUuid, serial: String, get_at: OffsetDateTime) -> Self {
        Self { serial, id: usb_id, get_at }
    }
}