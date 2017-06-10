
pub type DjSet = usize;

struct Node<'a>{
    payload: &'a Payload
}

pub struct DjSetContainer<'a> {
    nodes: Vec<Node<'a>>
}

impl <'a> PartialEq for DjSetContainer<'a> {
    fn eq(&self, other: &Self) -> bool {
        (self as * const Self) == (other as * const Self)
    }
}

type Payload = i32;

impl <'a> DjSetContainer<'a> {
    pub fn new() -> Self {
        DjSetContainer { nodes: Vec::new() }
    }

    pub fn add(&mut self, payload: &'a Payload) -> DjSet {
        let ret = self.nodes.len();
        let node = Node::<'a> { payload: payload };
        self.nodes.push(node);
        ret
    }

    pub fn find(&self, djset: &DjSet) -> DjSet {
        djset.clone()
    }

    pub fn union(&mut self, other: &Self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_elements_are_not_equivalent() {
        let p = 12;
        let mut dj = DjSetContainer::new();
        let a = dj.add(&p);
        let b = dj.add(&p);

        assert!(dj.find(&a) != dj.find(&b));
    }

    #[test]
    fn one_element_is_equivalent_to_itself() {
        let p = 1;
        let mut dj = DjSetContainer::new();
        let a = dj.add(&p);

        assert!(dj.find(&a) == dj.find(&a));
    }
//
//    #[test]
//    fn after_union_two_sets_should_become_equivalent() {
//        let a = DjSet::new(12);
//        let b = DjSet::new(12);
//
//        a.union(&b);
//
//        assert!(a.find() == b.find());
//    }
}