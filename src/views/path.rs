pub struct Path {
    pub prefix: String,
    pub backend: bool,
}

impl Path {
    pub fn define(&self, following_path: String) -> String {
        match self.backend {
            true => {
                let path: String = self.prefix.to_owned() + &following_path;
                String::from("api/v1") + &path
            }
            false => self.prefix.to_owned() + &following_path,
        }
    }
}
