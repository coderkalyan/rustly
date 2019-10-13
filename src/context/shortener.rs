use harsh::{Harsh, HarshBuilder};

pub struct Shortener {
    generator: Harsh,
}

impl Shortener {
    pub fn new() -> Shortener {
        let harsh = HarshBuilder::new().length(6).init().unwrap();
        Shortener {
            generator: harsh,
        }
    }

    pub fn hash(&self, id: u64) -> String {
        self.generator.encode(&[id]).unwrap()
    }
}
