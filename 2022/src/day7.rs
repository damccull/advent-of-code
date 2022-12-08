use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use advent_of_code_common::read_data_from_file;

const DEVICE_SPACE_NEEDED: u32 = 30000000;
const DEVICE_TOTAL_SPACE: u32 = 70000000;

fn main() -> Result<(), anyhow::Error> {
    let data = process_data(read_data_from_file("data/day7.txt")?)?;

    let result = size_of_directories_smaller_than_100000(Rc::new(data.clone()))
        .ok_or_else(|| anyhow::anyhow!("Unable to determine size"))?;

    println!(
        "The total size of all directories with less than 100000 is {}",
        result
    );

    let result =
        size_of_directory_to_delete(Rc::new(data), DEVICE_SPACE_NEEDED, DEVICE_TOTAL_SPACE)?;

    println!("The size of the directory to delete is {}", result);

    Ok(())
}

fn size_of_directory_to_delete(
    filesystem: Rc<FilesystemEntity>,
    space_needed: u32,
    total_space: u32,
) -> Result<u32, anyhow::Error> {
    let target_size = space_needed - (total_space - filesystem.size());

    let mut flat = flatten_directory_tree(filesystem)?;
    flat.sort_by_key(|d| d.size());

    let result = flat
        .iter()
        .find(|d| d.size() >= target_size)
        .ok_or_else(|| anyhow::anyhow!("No directory meets the size requirement"))?;
    Ok(result.size())
}

fn flatten_directory_tree(
    filesystem: Rc<FilesystemEntity>,
) -> Result<Vec<Rc<FilesystemEntity>>, anyhow::Error> {
    let mut result = Vec::new();
    result.push(filesystem.clone());

    let children = filesystem
        .children()
        .ok_or_else(|| anyhow::anyhow!("Couldn't borrow children"))?;
    let children = children.borrow();

    for entity in children.iter() {
        match entity.as_ref() {
            FilesystemEntity::File { .. } => {}
            FilesystemEntity::Directory { .. } => {
                result.append(&mut flatten_directory_tree(entity.clone())?);
            }
        }
    }

    Ok(result)
}

fn size_of_directories_smaller_than_100000(filesystem: Rc<FilesystemEntity>) -> Option<u32> {
    let Ok(flat) = flatten_directory_tree(filesystem) else {
        return None;
    };

    let mut size = 0_u32;
    for entity in flat.iter() {
        let x = entity.size();
        if entity.name() != "/" && x < 100000 {
            size += x;
        }
    }
    Some(size)
}

fn process_data(data: Vec<String>) -> Result<FilesystemEntity, anyhow::Error> {
    let filesystem = Rc::new(FilesystemEntity::default());
    let mut current_directory = filesystem.clone();
    for line in data {
        if line.starts_with('$') {
            if let Some(line) = line.strip_prefix("$ ") {
                if let Some(path) = line.strip_prefix("cd ") {
                    // Create new FilesytemEntity at path or move to existing one
                    match path {
                        "/" => current_directory = filesystem.clone(),
                        ".." => {
                            let Some(parent) = current_directory.parent() else{
                                anyhow::bail!("Already at root, can't go up any further");
                            };
                            let Some(parent) = parent.upgrade() else {
                                anyhow::bail!("Unable to get reference to parent");
                            };
                            current_directory = parent.clone();
                        }
                        _ => {
                            //See if the directory already exists in current directory
                            let dir = match current_directory.as_ref() {
                                FilesystemEntity::Directory { contents, .. } => {
                                    contents.borrow().iter().find(|f| f.name() == path).cloned()
                                }
                                _ => {
                                    anyhow::bail!("Should never reach this");
                                }
                            };
                            if let Some(dir) = dir {
                                current_directory = dir.clone();
                            } else {
                                //Create directory, add it to current directory, and enter it
                                let newdir = Rc::new(FilesystemEntity::Directory {
                                    name: path.to_string(),
                                    contents: RefCell::new(Vec::new()),
                                    parent: Some(Rc::downgrade(&current_directory)),
                                });
                                current_directory.add_to_contents(newdir.clone())?;
                                current_directory = newdir.clone();
                            }
                        }
                    }
                } else if line.starts_with("ls") {
                    // Don't actually care about the ls command itself, just the output after it
                    continue;
                } else {
                    anyhow::bail!("Unrecognized command")
                }
            }
        } else {
            // Handle `ls` output
            let (dir_or_size, name) = line
                .split_once(' ')
                .ok_or_else(|| anyhow::anyhow!("Badly formed data"))?;

            if dir_or_size == "dir" {
                // Insert new directory
                let newdir = Rc::new(FilesystemEntity::Directory {
                    name: name.to_string(),
                    contents: RefCell::new(Vec::new()),
                    parent: Some(Rc::downgrade(&current_directory)),
                });
                current_directory.add_to_contents(newdir.clone())?;
            } else {
                // Insert new file
                let newdir = Rc::new(FilesystemEntity::File {
                    name: name.to_string(),
                    size: dir_or_size.parse()?,
                    parent: Rc::downgrade(&current_directory),
                });
                current_directory.add_to_contents(newdir.clone())?;
            }
        }
    }

    let filesystem = (*filesystem).clone();

    Ok(filesystem)
}

#[derive(Debug, Clone)]
enum FilesystemEntity {
    File {
        name: String,
        size: u32,
        parent: Weak<FilesystemEntity>,
    },
    Directory {
        name: String,
        contents: RefCell<Vec<Rc<FilesystemEntity>>>,
        parent: Option<Weak<FilesystemEntity>>,
    },
}

impl FilesystemEntity {
    pub fn name(&self) -> String {
        match self {
            FilesystemEntity::File { name, .. } => name.clone(),
            FilesystemEntity::Directory { name, .. } => name.clone(),
        }
    }

    pub fn parent(&self) -> Option<Weak<FilesystemEntity>> {
        match self {
            FilesystemEntity::File { parent, .. } => Some(parent.clone()),
            FilesystemEntity::Directory { parent, .. } => parent.clone(),
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            FilesystemEntity::File { size, .. } => *size,
            FilesystemEntity::Directory { contents, .. } => {
                contents.borrow().iter().map(|c| c.size()).sum()
            }
        }
    }

    pub fn add_to_contents(&self, entity: Rc<FilesystemEntity>) -> Result<(), anyhow::Error> {
        match self {
            FilesystemEntity::Directory { contents, .. } => {
                contents.borrow_mut().push(entity);
                Ok(())
            }
            _ => {
                anyhow::bail!("Can't use this on a FilesytemEntity::File")
            }
        }
    }

    pub fn children(&self) -> Option<RefCell<Vec<Rc<FilesystemEntity>>>> {
        match self {
            FilesystemEntity::Directory { contents, .. } => Some(contents.clone()),
            _ => None,
        }
    }
}

impl Default for FilesystemEntity {
    fn default() -> Self {
        Self::Directory {
            name: "/".to_string(),
            contents: RefCell::new(Vec::new()),
            parent: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        flatten_directory_tree, process_data, size_of_directories_smaller_than_100000,
        size_of_directory_to_delete, DEVICE_SPACE_NEEDED, DEVICE_TOTAL_SPACE,
    };

    fn test_data() -> Vec<String> {
        r##"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"##
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
    }

    #[test]
    fn process_data_works() {
        let data = process_data(test_data()).unwrap();

        assert_eq!(48381165, data.size());
    }

    #[test]
    fn size_of_directories_smaller_than_100000_is_correct() {
        let data = Rc::new(process_data(test_data()).unwrap());

        let result = size_of_directories_smaller_than_100000(data).unwrap();

        assert_eq!(result, 95437);
    }

    #[test]
    fn size_of_directory_to_delete_works() {
        let data = Rc::new(process_data(test_data()).unwrap());

        let result = size_of_directory_to_delete(data, DEVICE_SPACE_NEEDED, DEVICE_TOTAL_SPACE);

        assert_eq!(result.unwrap(), 24933642)
    }

    #[test]
    fn flatten_directory_tree_works() {
        let data = Rc::new(process_data(test_data()).unwrap());

        let result = flatten_directory_tree(data).unwrap();

        assert_eq!(result.len(), 4);
    }
}
