use std::{fs, iter, path::PathBuf};

pub struct ShaderEntry(Option<PathBuf>, Option<PathBuf>, Option<PathBuf>);

pub fn index_resources(root: &PathBuf) -> Vec<(String, String, PathBuf)> {
    let mut resource_indexes = vec![];
    index_resources_rec(root, root, &mut resource_indexes);
    resource_indexes
}

fn index_resources_rec(
    root: &PathBuf,
    pwd: &PathBuf,
    resource_indexes: &mut Vec<(String, String, PathBuf)>,
) {
    if let Ok(entries) = fs::read_dir(pwd.as_path()) {
        let (resources, subdirs): (Vec<_>, Vec<_>) = entries
            .filter_map(|e| e.ok())
            .filter(|e| fs::metadata(e.path()).map_or(false, |m| m.is_dir()))
            .partition(|e| e.path().extension().is_some());

        let mut new_resources: Vec<_> = resources
            .iter()
            .filter_map(|resource| {
                let ext = String::from(resource.path().extension()?.to_str()?);
                let basename = String::from(resource.path().file_stem()?.to_str()?);
                let path = pwd.strip_prefix(root).ok()?;
                let res_id: Vec<_> = path
                    .iter()
                    .filter_map(|segment| segment.to_str())
                    .chain(iter::once(&basename as &str))
                    .collect();

                Some((res_id.join("."), ext, resource.path()))
            })
            .collect();

        resource_indexes.append(&mut new_resources);

        subdirs
            .iter()
            .for_each(|subdir| index_resources_rec(root, &subdir.path(), resource_indexes));
    }
}
