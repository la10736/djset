
#[derive(Debug)]
pub struct DjSet {}

impl PartialEq for DjSet {
    fn eq(&self, other: &Self) -> bool {
        (self as * const Self) == (other as * const Self)
    }
}

pub type Payload = i32;

impl DjSet {
    pub fn new(_content: Payload) -> Self {
        DjSet { }
    }

    pub fn find(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_elements_are_not_equivalent() {
        let a = DjSet::new(12);
        let b = DjSet::new(12);

        assert!(a.find() != b.find());
    }

    #[test]
    fn one_element_is_equivalent_to_itself() {
        let a = DjSet::new(1);

        assert_eq!(a.find(), a.find());
    }
}