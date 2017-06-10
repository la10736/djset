
pub type DjSet = usize;

struct Node<'a>{
    payload: &'a Payload,
    parent: Option<DjSet>
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
        let node = Node::<'a> { payload: payload, parent: None };
        self.nodes.push(node);
        ret
    }

    pub fn find(&self, djset: &DjSet) -> DjSet {
        if let Some(parent) = self.nodes[*djset].parent {
            return self.find(&parent);
        }
        djset.clone()
    }

    pub fn union(&mut self, left: &DjSet, right: &DjSet) {
        let left = self.find(left);
        let right = self.find(right);

        self.merge(left, right);
    }

    fn merge(&mut self, n0: DjSet, n1: DjSet) {
        self.nodes[n0].parent = Some(n1)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    static P: Payload = 12;

    #[test]
    fn two_elements_are_not_equivalent() {
        let mut dj = DjSetContainer::new();
        let a = dj.add(&P);
        let b = dj.add(&P);

        assert!(dj.find(&a) != dj.find(&b));
    }

    #[test]
    fn one_element_is_equivalent_to_itself() {
        let mut dj = DjSetContainer::new();
        let a = dj.add(&P);

        assert!(dj.find(&a) == dj.find(&a));
    }

    #[test]
    fn after_union_two_sets_should_become_equivalent() {
        let mut dj = DjSetContainer::new();
        let a = dj.add(&P);
        let b = dj.add(&P);

        dj.union(&a, &b);

        assert!(dj.find(&a) == dj.find(&b));
    }
}