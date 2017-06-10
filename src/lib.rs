
pub type DjSet = usize;

struct Node{
    parent: Option<DjSet>
}

pub struct DjSetContainer {
    nodes: Vec<Node>
}

impl PartialEq for DjSetContainer {
    fn eq(&self, other: &Self) -> bool {
        (self as * const Self) == (other as * const Self)
    }
}

impl DjSetContainer {
    pub fn new() -> Self {
        DjSetContainer { nodes: Vec::new() }
    }

    pub fn add(&mut self) -> DjSet {
        let ret = self.nodes.len();
        let node = Node { parent: None };
        self.nodes.push(node);
        ret
    }

    pub fn find(&mut self, djset: DjSet) -> DjSet {
        match self.nodes[djset].parent {
            Some(parent) => {
                self.find(parent)
            }
            None => djset.clone()
        }
    }

    pub fn union(&mut self, left: DjSet, right: DjSet) {
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

    #[test]
    fn two_elements_are_not_equivalent() {
        let mut dj = DjSetContainer::new();
        let a = dj.add();
        let b = dj.add();

        assert!(dj.find(a) != dj.find(b));
    }

    #[test]
    fn one_element_is_equivalent_to_itself() {
        let mut dj = DjSetContainer::new();
        let a = dj.add();

        assert!(dj.find(a) == dj.find(a));
    }

    #[test]
    fn after_union_two_sets_should_become_equivalent() {
        let mut dj = DjSetContainer::new();
        let a = dj.add();
        let b = dj.add();

        dj.union(a, b);

        assert!(dj.find(a) == dj.find(b));
    }


}