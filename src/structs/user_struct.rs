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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_create() {
        let user = User {
            name: String::from("Jorge"),
            last_name: String::from("Carlos")
        };

        assert_eq!(user.full_name(), "Jorge Carlos")
    }
}