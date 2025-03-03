use super::*;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;
struct FileSystem {
    image_path: String,
}

#[async_trait]
impl AsyncFilesystem for FileSystem {
    async fn init(path: String) -> Self {
        Self { image_path: path }
    }

    async fn get_image(&self, name: String) -> Result<Vec<u8>, FsError> {
        let path = PathBuf::from(&self.image_path).join(&name);
        match fs::read(path).await {
            Ok(image_stream) => Ok(image_stream),
            Err(_) => Err(FsError::ReadError(name)),
        }
    }

    async fn save_image(&self, name: String, image_stream: Vec<u8>) -> Result<(), FsError> {
        let path = PathBuf::from(&self.image_path).join(&name);

        match fs::write(path, &image_stream).await {
            Ok(image_stream) => Ok(image_stream),
            Err(_) => Err(FsError::WriteError(name)),
        }
    }

    async fn del_image(&self, name: String) -> Result<(), FsError> {
        let path = PathBuf::from(&self.image_path).join(&name);
        match fs::try_exists(&path).await {
            Ok(boolean) => match boolean {
                true => match fs::remove_file(path).await {
                    Ok(()) => Ok(()),
                    Err(_) => Err(FsError::DeleteError(name)),
                },
                false => Err(FsError::ReadError(name)),
            },
            Err(_) => Err(FsError::ReadError(name)),
        }
    }

    fn validate_filename(name: &str) -> Result<(), FsError> {
        if name.contains("..") || name.starts_with('/') {
            return Err(FsError::InvalidName(name.to_string()));
        }
        Ok(())
    }

    // async fn setup_directory(&self) -> Result<(), FsError>{}
}
