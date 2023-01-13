#[derive(Debug)]
pub struct User {
    pub name: String,
    pub last_name: String,
}

impl User {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.name, self.last_name)
    }
}