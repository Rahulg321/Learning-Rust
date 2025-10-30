#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn grow(&mut self, w: u32, h: u32) {
        self.width += w;
        self.height += h;
    }

    // this function takes ownership of the Rectangle instance
    pub fn into_square(self) -> Rectangle {
        let side = self.width.min(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }

    pub fn can_hold(&self, other_rec: &Rectangle) -> bool {
        if self.width > other_rec.width && self.height > other_rec.height {
            println!("yes it can hold");
            return true;
        } else {
            false
        }
    }
}
