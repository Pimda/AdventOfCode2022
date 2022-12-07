use std::fmt::Display;

fn main() {
    let root = build_tree_from_file("input.txt");

    print!("{}", root);

    println!("part 1: {}", part_1(&root));
    println!("part 2: {}", part_2(&root));
}

fn build_tree_from_file(filename: &str) -> Folder {
    let commands = read_input(filename);
    let mut root = Folder::new();
    handle_commands(&mut root, &mut commands.iter().skip(1));
    root
}

fn part_1(root: &Folder) -> u64 {
    let size_list = root.get_size_list();

    size_list.iter().filter(|f| **f < 100000).sum()
}

fn part_2(root: &Folder) -> u64 {
    let total_space = 70000000;
    let required_space = 30000000;
    let used_space = root.get_size();
    let free_space = total_space - used_space;
    let cleanup_space = required_space - free_space;

    let mut size_list = root.get_size_list();

    size_list.sort();
    *size_list.iter().find(|f| **f >= cleanup_space).unwrap()
}

fn handle_commands<'a, 'b, 'c>(
    current_folder: &'a mut Folder,
    commands: &'b mut impl Iterator<Item = &'c String>,
) {
    loop {
        let command = commands.next();

        match command {
            None => return,
            Some(command) => {
                let input = command.split(' ').collect::<Vec<&str>>();

                match input[..] {
                    [left, right] => match (left, right) {
                        ("$", "ls") => {
                            // skip
                        }
                        ("dir", name) => current_folder.add_folder_if_not_exists(name.to_owned()),
                        (size, name) => current_folder
                            .files
                            .push(File::new(name.to_owned(), size.parse::<u64>().unwrap())),
                    },
                    ["$", "cd", path] => match path {
                        ".." => {
                            return;
                        }
                        name => {
                            current_folder.add_folder_if_not_exists(name.to_owned());
                            let folder = current_folder.get_folder(name.to_owned());
                            handle_commands(folder, commands)
                        }
                    },
                    _ => panic!("Invalid commands"),
                }
            }
        }
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).expect("File not found");
    input.lines().map(|l| l.to_owned()).collect()
}

struct Folder {
    name: String,
    folders: Vec<Folder>,
    files: Vec<File>,
}

impl Folder {
    fn new() -> Self {
        Folder {
            name: "/".to_owned(),
            folders: vec![],
            files: vec![],
        }
    }

    fn with_name(name: String) -> Self {
        Folder {
            name,
            folders: vec![],
            files: vec![],
        }
    }

    fn add_folder_if_not_exists(&mut self, name: String) {
        let folder = self.folders.iter_mut().find(|f| f.name == name);

        match folder {
            None => {
                let new_folder = Folder::with_name(name);
                self.folders.push(new_folder);
            }
            Some(_) => (),
        }
    }

    fn get_folder(&mut self, name: String) -> &mut Folder {
        let folder = self.folders.iter_mut().find(|f| f.name == name);
        folder.unwrap()
    }

    fn get_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum::<u64>()
            + self.folders.iter().map(|f| f.get_size()).sum::<u64>()
    }

    fn get_size_list(&self) -> Vec<u64> {
        let mut list: Vec<u64> = self
            .folders
            .iter()
            .flat_map(|f| f.get_size_list())
            .collect();
        list.push(self.get_size());
        list
    }

    fn print(&self, depth: usize) {
        println!("{}- {} (dir)", "  ".repeat(depth), self.name);

        for file in &self.files {
            println!(
                "{}- {} (file, size={})",
                "  ".repeat(depth + 1),
                file.name,
                file.size
            );
        }

        for folder in &self.folders {
            folder.print(depth + 1);
        }
    }
}

impl Display for Folder {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(0);
        Ok(())
    }
}

struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> Self {
        File { name, size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let root = build_tree_from_file("test.txt");
        assert_eq!(part_1(&root), 95437);
    }

    #[test]
    fn part_2_works() {
        let root = build_tree_from_file("test.txt");
        assert_eq!(part_2(&root), 24933642);
    }
}
