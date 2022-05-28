use log::info;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MinecraftError {
    /// An Error caused by I/O
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A VarError caused by env
    #[error(transparent)]
    Env(#[from] std::env::VarError),

    /// An Error caused by Zip
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}

pub type MinecraftResult<T> = Result<T, MinecraftError>;

#[cfg(windows)]
pub fn vanilla_root_path() -> MinecraftResult<PathBuf> {
    Ok(PathBuf::from(&std::env::var("appdata")?).join(".minecraft"))
}
#[cfg(target_os = "linux")]
pub fn vanilla_root_path() -> MinecraftResult<PathBuf> {
    Ok(PathBuf::from(&std::env::var("HOME")?).join(".minecraft"))
}
#[cfg(target_os = "macos")]
pub fn vanilla_root_path() -> MinecraftResult<PathBuf> {
    Ok(PathBuf::from(&std::env::var("HOME")?)
        .join("Library")
        .join("Application Support")
        .join("cminecraft"))
}

pub fn fetch_assets(version: &str) -> MinecraftResult<()> {
    let buf = vanilla_root_path()?
        .join("versions")
        .join(version)
        .join(format!("{}.jar", version));

    info!("Opening {:?}...", &buf);
    let file = std::fs::File::open(&buf)?;
    let mut archive = zip::ZipArchive::new(file)?;
    info!("File {:?} contains {} files", &buf, archive.len());

    let mut count = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let path = sanitize_filename(file.name());
        if file.name().starts_with("assets/minecraft") {
            std::fs::create_dir_all(path.parent().unwrap())?;

            let mut outfile = std::fs::File::create(&path)?;
            std::io::copy(&mut file, &mut outfile)?;
            count += 1;
        }
    }

    info!("Extracted {} files.", count);
    Ok(())
}

fn sanitize_filename(filename: &str) -> PathBuf {
    PathBuf::from(match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    })
}
