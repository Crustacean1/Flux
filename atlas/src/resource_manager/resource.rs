#[derive(Clone)]
pub struct Resource<T: Clone> {
    id: String,
    pub res: T,
}

impl<T: Clone> Resource<T> {
    pub fn new(id: String, res: &T) -> Self {
        Resource::<T> {
            id: String::from(id),
            res: res.clone(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
