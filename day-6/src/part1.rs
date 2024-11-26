use std::{cell::RefCell, rc::Rc};

use ahash::{HashMap, HashMapExt};

#[derive(Default)]
struct Object {
    children: Vec<Rc<RefCell<Object>>>,
}

impl Object {
    fn new_with_child(child: Rc<RefCell<Object>>) -> Self {
        Self {
            children: vec![child],
        }
    }
}

fn count_object(mut depth: usize, object: Rc<RefCell<Object>>) -> usize {
    depth += 1;
    let mut count = 0;
    for child in object.borrow().children.clone() {
        count += depth;
        count += count_object(depth, child);
    }
    count
}

fn load_map(input: &str) -> HashMap<String, Rc<RefCell<Object>>> {
    let mut map: HashMap<String, Rc<RefCell<Object>>> = HashMap::new();
    input
        .lines()
        .map(|line| line.trim().split_once(')').unwrap())
        .for_each(|(parent, child)| {
            let child = map.entry(child.to_string()).or_default().clone();
            map.entry(parent.to_string())
                .and_modify(|object| object.borrow_mut().children.push(child.clone()))
                .or_insert(Rc::new(RefCell::new(Object::new_with_child(child))));
        });
    map
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let map = load_map(input);
    count_object(0, map.get("COM").unwrap().clone()).to_string()
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
K)L";
        assert_eq!("42", process(input));
    }
}
