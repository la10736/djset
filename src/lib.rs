
#[derive(PartialEq, Debug)]
pub struct DjSet {}

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

        assert_eq!(a.find(), b.find());
    }
}