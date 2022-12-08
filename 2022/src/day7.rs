use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fs::File,
    rc::{Rc, Weak},
};

fn main() -> Result<(), anyhow::Error> {
    Ok(())
}

fn process_data(data: Vec<String>) -> Result<FilesystemEntity, anyhow::Error> {
    let filesystem = Rc::new(FilesystemEntity::default());
    let mut current_directory = filesystem.clone();
    for line in data {
        if line.starts_with('$') {
            dbg!(&line);
            if let Some(line) = line.strip_prefix("$ ") {
                dbg!(&line);
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
                                    let x = contents.borrow();
                                    let y = x.iter().find(|f| f.name() == path);
                                    y
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
                                current_directory.add_to_contents(newdir.clone());
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
        }
    }

    dbg!(&filesystem);

    todo!()
}

enum Command {
    ChangeDirectory { location: String },
    ListFiles,
}

#[derive(Debug)]
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

    pub fn path(&self) -> String {
        let Some(parent) = self.parent() else{
            return String::default();
        };
        let Some(parent) = parent.upgrade() else {
            return String::default();
        };
        let name = match self {
            FilesystemEntity::File { name, .. } => name,
            FilesystemEntity::Directory { name, .. } => name,
        };

        format!("{}/{}", parent.path(), name)
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
                contents.get_mut().push(entity);
                Ok(())
            }
            _ => {
                anyhow::bail!("Can't use this on a FilesytemEntity::File")
            }
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
    use crate::process_data;

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

        println!("Size: {}", data.size());

        assert_eq!(1, 1);
    }
}
