#[derive(Clone)]
pub struct PoolIdentifier {
    name: String,
}

impl PoolIdentifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}
