use std::{cell::RefCell, rc::Rc};

use ahash::{HashMap, HashMapExt};

struct Object {
    name: String,
    parent: Option<Rc<RefCell<Object>>>,
    children: Vec<Rc<RefCell<Object>>>,
}

impl Object {
    fn new(name: impl ToString, parent: Option<Rc<RefCell<Object>>>) -> Self {
        Self {
            name: name.to_string(),
            parent,
            children: Vec::default(),
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
        }));
        child.borrow_mut().parent = Some(this.clone());
        this
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
    let map = load_map(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        todo!("Havent built test yet");
        let input = "";
        assert_eq!("", process(input));
    }
}
