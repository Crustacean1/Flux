pub struct Unit {
    pub name: String,
    pub unit_type: String,
    pub health: f32,
    pub max_health: f32,
}

impl Unit {
    pub fn new(name: &str, unit_type: &str, max_health: f32) -> Self {
        Self {
            name: String::from(name),
            unit_type: String::from(unit_type),
            health: max_health,
            max_health,
        }
    }
}
