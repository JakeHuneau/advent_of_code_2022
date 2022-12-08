#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fs::{read_to_string, File},
    ops::DerefMut,
    rc::Rc,
    str::FromStr,
    thread::current,
};

pub fn solve() -> SolutionPair {
    // let mut filestructure = FileObject {
    //     name: String::from("/"),
    //     size: 0,
    //     parent: None,
    //     children: vec![],
    // };
    // // let mut head = Rc::new(&filestructure);
    // let mut current_directory = Rc::new(RefCell::new(filestructure));
    // read_to_string("input/day7")
    //     .expect("Could not read file")
    //     .lines()
    //     .for_each(|line| {
    //         let line_parts = line.split_whitespace().collect::<Vec<&str>>();
    //         if line_parts[0] == "$" && line_parts[1] == "cd" && line_parts[2] != "/" {
    //             match current_directory.into_inner().cd(line_parts[2]) {
    //                 Some(new_directory) => current_directory = new_directory,
    //                 None => {}
    //             }
    //         } else {
    //             // Must be a fileobject description
    //             current_directory
    //                 .get_mut()
    //                 .add_child(line.parse::<FileObject>().unwrap());
    //         }
    //     });
    // println!("{:?}", current_directory.get_mut().name);
    (Solution::UInt(1), Solution::UInt(1))
}

// struct FileObject {
//     name: String,
//     size: usize,
//     parent: Option<Rc<RefCell<FileObject>>>,
//     children: Vec<Rc<RefCell<FileObject>>>,
// }

// impl FromStr for FileObject {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut split = s.split_whitespace().collect::<Vec<&str>>();
//         match split[0] == "dir" {
//             true => Ok(FileObject::new(String::from(split[1]), 0)),
//             false => Ok(FileObject::new(
//                 String::from(split[1]),
//                 split[0].parse::<usize>().unwrap(),
//             )),
//         }
//     }
// }

// impl FileObject {
//     fn new(name: String, size: usize) -> Self {
//         Self {
//             name,
//             size,
//             parent: None,
//             children: vec![],
//         }
//     }

//     fn get_size(&self) -> usize {
//         self.size
//             + self
//                 .children
//                 .iter()
//                 .map(|child| child.get_mut().get_size())
//                 .sum::<usize>()
//     }

//     fn cd(&self, new_location: &str) -> Option<Rc<RefCell<FileObject>>> {
//         if new_location == ".." {
//             return match &self.parent {
//                 Some(parent) => Some(parent.to_owned()),
//                 None => None,
//             };
//         }
//         for dir in &self.children {
//             if dir.get_mut().name == new_location {
//                 return Some(dir.to_owned());
//             }
//         }
//         None
//     }

//     fn add_parent(&mut self, parent: Rc<RefCell<FileObject>>) {
//         self.parent = Some(parent);
//     }

//     fn add_child(&mut self, child: FileObject) {
//         self.children.push(Rc::new(RefCell::new(child)));
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_dir_size_no_child_dir() {
//         let dir = FileObject {
//             name: String::from("dir"),
//             size: 0,
//             parent: None,
//             children: vec![
//                 Rc::new(RefCell::new(FileObject::new(String::from("a"), 1))),
//                 Rc::new(RefCell::new(FileObject::new(String::from("b"), 2))),
//             ],
//         };
//         assert_eq!(dir.get_size(), 3);
//     }

//     #[test]
//     fn test_dir_size_with_child_dir() {
//         let dir = FileObject {
//             name: String::from("dir1"),
//             size: 0,
//             parent: None,
//             children: vec![
//                 Rc::new(RefCell::new(FileObject::new(String::from("a"), 1))),
//                 Rc::new(RefCell::new(FileObject::new(String::from("b"), 2))),
//                 Rc::new(RefCell::new(FileObject {
//                     name: String::from("child"),
//                     size: 0,
//                     parent: None,
//                     children: vec![
//                         Rc::new(RefCell::new(FileObject::new(String::from("c"), 3))),
//                         Rc::new(RefCell::new(FileObject::new(String::from("d"), 4))),
//                     ],
//                 })),
//             ],
//         };
//         assert_eq!(dir.get_size(), 10);
//     }

//     #[test]
//     fn test_add_child() {
//         let mut parent = FileObject::new(String::from("parent"), 0);
//         let child = FileObject::new(String::from("child"), 0);
//         parent.add_child(child);
//         assert_eq!(parent.children[0].get_mut().name, "child");
//     }

//     #[test]
//     fn test_add_parent() {
//         let parent = FileObject::new(String::from("parent"), 0);
//         let mut child = FileObject::new(String::from("child"), 0);
//         child.add_parent(Rc::new(RefCell::new(parent)));
//         assert_eq!(child.parent.unwrap().get_mut().name, "parent");
//     }

//     #[test]
//     fn test_cd_down() {
//         let dir = FileObject {
//             name: String::from("parent"),
//             size: 0,
//             parent: None,
//             children: vec![Rc::new(RefCell::new(FileObject::new(
//                 String::from("child"),
//                 0,
//             )))],
//         };
//         let mut current_dir = Rc::new(RefCell::new(dir));
//         current_dir = current_dir.get_mut().cd("child").unwrap();
//         assert_eq!(*current_dir.get_mut().name, String::from("child"));
//     }

//     #[test]
//     fn test_cd_up() {
//         let parent = FileObject::new(String::from("parent"), 0);
//         let mut child = FileObject {
//             name: String::from("child"),
//             size: 0,
//             parent: Some(Rc::new(RefCell::new(parent))),
//             children: vec![],
//         };
//         let mut current_dir = Rc::new(RefCell::new(child));
//         current_dir = current_dir.get_mut().cd("..").unwrap();
//         assert_eq!(*current_dir.get_mut().name, String::from("parent"));
//     }
// }
