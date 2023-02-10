use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    total_size: RefCell<usize>,
    files: RefCell<Vec<Rc<File>>>,
    children: RefCell<Vec<Rc<Directory>>>,
    root: Option<Weak<Directory>>,
    parent: Option<Weak<Directory>>,
}

impl Directory {
    fn new(
        Directory {
            root,
            name,
            total_size,
            files,
            children,
            parent,
        }: Directory,
    ) -> Rc<Directory> {
        Rc::new(Directory {
            root,
            name,
            total_size,
            files,
            children,
            parent,
        })
    }
    fn add_new_file(&self, file_name: &str, file_size: usize) {
        // i think there is some weird that stops me from inline dereferencing * and i have to use
        // drop
        let mut current_dir_ptr = self.files.borrow_mut();
        current_dir_ptr.push(Rc::new(File {
            name: file_name.to_string(),
            size: file_size,
        }));
        drop(current_dir_ptr);
        self.increment_total_size(file_size);
    }

    fn increment_total_size(&self, size: usize) {
        let mut total_size = self.total_size.borrow_mut();
        *total_size += size;

        if let Some(parent) = &self.parent {
            parent.upgrade().unwrap().increment_total_size(size);
        }
    }
}

fn main() {
    let sample_ops = "
$ cd /
$ ls
dir blrnnv
dir ctfjwl
dir dqf
135993 dqqbcfr
dir ftj
125510 fzjdz
dir jvtlfbzr
31762 lsvw.lwr
dir qfstpm
dir sbprmc
dir svbnljr
dir tchbjclg
dir wtm
dir ztrz
$ cd blrnnv
$ ls
169869 mjjj.wnq
$ cd ..
$ cd ctfjwl
";

    let cd_re = Regex::new(r"\$ cd (?P<directory_name>.*)").unwrap();
    let ls_re = Regex::new(r"\$ ls").unwrap();
    let dir_re = Regex::new(r"dir (?P<directory_name>.*)").unwrap();
    let file_re = Regex::new(r"(?P<file_size>\d*) (?P<file_name>.*)").unwrap();

    let ops = include_str!("./day7.txt").split('\n');
    // let ops = sample_ops.split('\n');
    let root = Rc::new(Directory {
        name: "/".to_string(),
        parent: None,
        root: None,
        children: RefCell::new(vec![]),
        total_size: RefCell::new(0),
        files: RefCell::new(vec![]),
    });
    let mut current_dir = Rc::clone(&root);

    for op in ops {
        if cd_re.is_match(op) {
            let captured_text = cd_re.captures(op).unwrap();
            let directory_name = captured_text.name("directory_name").unwrap().as_str();
            if directory_name == "/" && current_dir.name != "/" {
                current_dir = current_dir.root.as_ref().unwrap().upgrade().unwrap();
            };
            if directory_name == ".." && current_dir.parent.is_some() {
                current_dir = current_dir.parent.as_ref().unwrap().upgrade().unwrap();
            }
            if current_dir
                .children
                .borrow()
                .iter()
                .any(|child_dir| child_dir.name == directory_name)
            {
                let child_dir = current_dir
                    .children
                    .borrow()
                    .iter()
                    .find(|child_dir| child_dir.name == directory_name)
                    .unwrap()
                    .clone();

                current_dir = Rc::clone(&child_dir);
            }

            // println!("cd {}", op);
            // println!("directory changed {:#?}", current_dir);
            continue;
        }
        if dir_re.is_match(op) {
            // println!("dir {}", op);
            let captured_text = dir_re.captures(op).unwrap();
            let directory_name = captured_text.name("directory_name").unwrap().as_str();
            if !current_dir
                .children
                .borrow()
                .iter()
                .any(|child_dir| child_dir.name == directory_name)
            {
                let new_dir = Directory::new(Directory {
                    name: directory_name.to_string(),
                    parent: Some(Rc::downgrade(&current_dir)),
                    root: Some(Rc::downgrade(&root)),
                    files: RefCell::new(vec![]),
                    children: RefCell::new(vec![]),
                    total_size: RefCell::new(0),
                });

                let mut current_dir_ptr = current_dir.children.borrow_mut();
                current_dir_ptr.push(new_dir);
                drop(current_dir_ptr);

                // println!("directory created");
            } else {
                // println!("directory already exists");
            }
            continue;
        }

        if ls_re.is_match(op) {
            // println!("ls {}", op);
            continue;
        }
        if file_re.is_match(op) {
            let captured_text = file_re.captures(op).unwrap();
            let file_size: usize = captured_text
                .name("file_size")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            let file_name = captured_text.name("file_name").unwrap().as_str();

            current_dir.add_new_file(file_name, file_size);

            // println!("file created {:?}", current_dir.total_size);
        }
    }

    current_dir = Rc::clone(&root);
    let sum_of_dirs_under_a_hundji = get_childrens_file_sizes(&current_dir.children, 0);

    fn get_childrens_file_sizes(
        children: &RefCell<Vec<Rc<Directory>>>,
        mut current_sum: usize,
    ) -> usize {
        children.borrow().iter().for_each(|child_dir| {
            let current_dir_size = *child_dir.total_size.borrow();

            if current_dir_size < 100000 {
                current_sum += current_dir_size;
            }
            current_sum = get_childrens_file_sizes(&child_dir.children, current_sum);
        });
        current_sum
    }

    fn find_size_of_smallest_directory_to_delete(
        children: &RefCell<Vec<Rc<Directory>>>,
        mut current_guess: usize,
        value_to_meet: usize,
    ) -> usize {
        children.borrow().iter().for_each(|child_dir| {
            let current_dir_size = *child_dir.total_size.borrow();
            println!("current guess {:?}", current_guess);
            println!("value_to_meet {:?} - current_guess {:?} > value_to_meet {:?} - current_dir_size  {:?}", value_to_meet, current_guess, value_to_meet, current_dir_size);
            if current_dir_size < value_to_meet {
                return
            }

            if  current_dir_size > value_to_meet && current_dir_size < current_guess {
                current_guess = current_dir_size
            }

            current_guess = find_size_of_smallest_directory_to_delete(
                &child_dir.children,
                current_guess,
                value_to_meet,
            );
        });
        current_guess
    }
    let file_system_excess_memory = *current_dir.total_size.borrow() - 40000000;
    println!(
        "smallest file to delete {:?}",
        find_size_of_smallest_directory_to_delete(
            &current_dir.children,
            *current_dir.total_size.borrow(),
            file_system_excess_memory
        )
    );
    println!("total size {:?}", current_dir.total_size);
    println!("overflow {:?}", file_system_excess_memory);
}
