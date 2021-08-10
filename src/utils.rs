pub trait Pad {
    fn pad(self, des_len: usize) -> Self;
    fn pad_c(self, des_len: usize) -> Self;
}

impl Pad for String {
    fn pad(mut self, des_len: usize) -> Self {
        let diff = des_len - self.len();

        let mut pad = String::new();
        for _ in 0..diff {
            pad.push(' ');
        }
        self.push_str(&pad);
        self
    }
    fn pad_c(self, des_len: usize) -> Self {
        let diff = des_len - self.len();

        let (mut pre, mut post) = (String::new(), String::new());
        for _ in 0..(diff/2) {
            pre.push(' ');
        }
        for _ in 0..(diff/2 + diff%2) {
            post.push(' ');
        }
        format!("{}{}{}", pre, self, post)
    }
}