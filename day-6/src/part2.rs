use std::{cell::RefCell, rc::Rc};

use ahash::{HashMap, HashMapExt};
use itertools::Itertools;

struct Object {
    name: String,
    parent: Option<Rc<RefCell<Object>>>,
    children: Vec<Rc<RefCell<Object>>>,
    depth: usize,
}

impl Object {
    fn new(name: impl ToString, parent: Option<Rc<RefCell<Object>>>) -> Self {
        Self {
            name: name.to_string(),
            parent,
            children: Vec::default(),
            depth: 0,
        }
    }

    fn new_with_new_child(
        name: impl ToString,
        parent: Option<Rc<RefCell<Object>>>,
        child: impl ToString,
    ) -> (Rc<RefCell<Self>>, Rc<RefCell<Self>>) {
        let this = Rc::new(RefCell::new(Self::new(name, parent)));
        let child = Rc::new(RefCell::new(Self::new(child, Some(this.clone()))));
        this.borrow_mut().children.push(child.clone());
        (this, child)
    }

    fn new_with_child(
        name: impl ToString,
        parent: Option<Rc<RefCell<Object>>>,
        child: Rc<RefCell<Object>>,
    ) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            name: name.to_string(),
            parent,
            children: vec![child.clone()],
            depth: 0,
        }));
        child.borrow_mut().parent = Some(this.clone());
        this
    }

    fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
        self.print();
        assert_eq!(self.depth == 0, self.parent.is_none());
        for child in &self.children {
            child.borrow_mut().set_depth(self.depth + 1);
        }
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Object>>> {
        self.parent.clone()
    }

    fn print(&self) {
        println!(
            "Node: {}, Depth: {}, Parent: {:?}, Children: {:?}",
            self.name,
            self.depth,
            self.parent.is_some(),
            self.children
                .iter()
                .cloned()
                .map(|child| child.borrow().name.clone())
                .collect_vec()
        );
    }
}

fn load_map(input: &str) -> HashMap<String, Rc<RefCell<Object>>> {
    let mut map: HashMap<String, Rc<RefCell<Object>>> = HashMap::new();
    input
        .lines()
        .map(|line| line.trim().split_once(')').unwrap())
        .for_each(|(parent_name, child_name)| {
            if let Some(parent) = map.get(parent_name) {
                let child = Rc::new(RefCell::new(Object::new(child_name, Some(parent.clone()))));
                parent.borrow_mut().children.push(child.clone());
                map.insert(child_name.to_string(), child);
            } else if let Some(child) = map.get(child_name) {
                map.insert(
                    parent_name.to_string(),
                    Object::new_with_child(parent_name, None, child.clone()),
                );
            } else {
                let (parent, child) = Object::new_with_new_child(parent_name, None, child_name);
                map.insert(parent_name.to_string(), parent);
                map.insert(child_name.to_string(), child);
            }
        });
    map
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    // Load the map from the input
    let map = load_map(input);
    // Fill out all the orbit depth values so that we can find an efficient path
    map.get("COM")
        .expect("COM must be present at the core of the map")
        .borrow_mut()
        .set_depth(0);
    // Find the efficient path
    let mut current_santa = map
        .get("SAN")
        .expect("Santa must be present on the map somewhere")
        .clone();
    let mut current_you = map
        .get("YOU")
        .expect("You must be present on the map somewhere")
        .clone();
    let mut steps = 0usize;
    let mut cont = true;
    while cont {
        match current_santa
            .clone()
            .borrow()
            .depth
            .cmp(&current_you.clone().borrow().depth)
        {
            std::cmp::Ordering::Equal => {
                // We're allowed to unwrap the parent values here because they cant be empty unless
                // something has gone terribly wrong
                if current_santa
                    .borrow()
                    .get_parent()
                    .unwrap_or_else(|| {
                        current_santa.borrow().print();
                        panic!("No parent! from santa's side");
                    })
                    .borrow()
                    .name
                    == current_you
                        .borrow()
                        .get_parent()
                        .unwrap_or_else(|| {
                            current_you.borrow().print();
                            panic!("No parent! from your side");
                        })
                        .borrow()
                        .name
                {
                    cont = false;
                } else {
                    current_santa = current_santa.clone().borrow().get_parent().unwrap();
                    current_you = current_you.clone().borrow().get_parent().unwrap();
                    steps += 2;
                }
            }
            std::cmp::Ordering::Less => {
                // Santa is less, you need to go down
                current_you = current_you.clone().borrow().get_parent().unwrap();
                steps += 1;
            }
            std::cmp::Ordering::Greater => {
                // You are less, santa needs to go down
                current_santa = current_santa.clone().borrow().get_parent().unwrap();
                steps += 1;
            }
        }
    }
    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
L)YOU
I)SAN";
        assert_eq!("5", process(input));
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!("4", process(input));
    }
}
