pub struct ResourceUnit {
    pub name: String,
}

impl ResourceUnit {
    pub fn new(name: String) -> ResourceUnit {
        ResourceUnit { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_important_in_resource_unit() {
        let unit = ResourceUnit::new(String::from("Test Unit"));

        assert_eq!("Test Unit", unit.name);
    }
}
