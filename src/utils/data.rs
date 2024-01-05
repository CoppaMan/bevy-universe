use {
    directories::ProjectDirs,
    std::{
        fs, io,
        path::{Path, PathBuf},
    },
};

pub enum DataDir {
    Base,
    Planets,
    Crafts,
}

impl ToString for DataDir {
    fn to_string(&self) -> String {
        match self {
            DataDir::Base => "".into(),
            DataDir::Planets => "planets".into(),
            DataDir::Crafts => "crafts".into(),
        }
    }
}

pub fn get_data_dir(dir: DataDir) -> PathBuf {
    ProjectDirs::from("com", "CoppaCom", "BevyPoc")
        .expect("")
        .data_dir()
        .join(dir.to_string())
}

pub fn create_data(src: PathBuf) {
    let dst = get_data_dir(DataDir::Base);
    fs::create_dir_all(&dst).expect("");
    copy_dir_all(&src, &dst).expect("");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
