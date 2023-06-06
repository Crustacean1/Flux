#[derive(Clone)]
pub struct Resource<T: Clone> {
    id: String,
    pub res: T,
}

impl<T: Clone> Resource<T> {
    pub fn new(id: &str, res: T) -> Self {
        Resource::<T> {
            id: String::from(id),
            res,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
