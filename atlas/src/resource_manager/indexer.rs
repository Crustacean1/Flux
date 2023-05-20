use std::{fs, path::PathBuf};

use crate::game_root::GameError;

pub struct ShaderEntry(Option<PathBuf>, Option<PathBuf>, Option<PathBuf>);

pub fn index_resources(root: &PathBuf) -> Result<Vec<(String, Vec<PathBuf>)>, GameError> {
    let subdirs: Vec<_> = list_entries(root)?
        .iter()
        .filter(|&entry| fs::metadata(entry.as_path()).map_or_else(|e| false, |m| m.is_dir()))
        .fold(
            Ok(vec![]),
            |resource_vectors: Result<Vec<(String, Vec<PathBuf>)>, GameError>, subdir| {
                let mut resource_vectors = resource_vectors?;
                let resource_files = crawl_dirs(subdir)?;
                resource_vectors.push((
                    String::from(subdir.file_stem().unwrap().to_str().unwrap()),
                    resource_files,
                ));
                Ok(resource_vectors)
            },
        )?;

    Ok(subdirs)
}

fn crawl_dirs(root: &PathBuf) -> Result<Vec<PathBuf>, GameError> {
    let local_resources = find_local_resources(root)?;

    let entries = fs::read_dir(root.as_path())?;

    entries
        .filter(|entry| match entry {
            Ok(entry) => match fs::metadata(entry.path()) {
                Ok(meta) => meta.is_dir(),
                _ => false,
            },
            _ => false,
        })
        .fold(Ok(local_resources), |resources, subdir| {
            let mut resources = resources?;
            resources.append(&mut crawl_dirs(&root.join(subdir?.path()))?);
            Ok(resources)
        })
}

fn find_local_resources(root: &PathBuf) -> Result<Vec<PathBuf>, GameError> {
    let Ok(entries) = fs::read_dir(root.as_path()) else {return Err(GameError::new("Failed to read directory contents"))};

    Ok(entries
        .map(|entry| entry.unwrap())
        .filter(|entry| fs::metadata(entry.path()).unwrap().is_file())
        .map(|file| file.path())
        .collect())
}

fn list_entries(path: &PathBuf) -> Result<Vec<PathBuf>, GameError> {
    let Ok(dir_entries) = fs::read_dir(path) else {return Err(GameError::new(&format!("Failed to read dir: {:?}", path)));};
    dir_entries.fold(Ok(vec![]), |entries, entry| match entries {
        Ok(mut entries) => match entry {
            Ok(entry) => {
                entries.push(entry.path());
                Ok(entries)
            }
            Err(e) => Err(GameError::new(&format!("Failed to read file {}\n", e))),
        },
        Err(e) => Err(e),
    })
}
