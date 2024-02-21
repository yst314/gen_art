pub fn euclidean_gcd(mut x0: u32, mut x1: u32) -> u32 {
    // 課題1.1
    while x1 != 0 {
        let remainder = x0 % x1;
        x0 = x1;
        x1 = remainder;
    }
    x0
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn quad_points(&self) -> [(f32, f32); 4] {
        let p0 = (self.x, self.y);
        let p1 = (self.x + self.w, self.y);
        let p2 = (self.x + self.w, self.y + self.h);
        let p3 = (self.x, self.y + self.h);
        [p0, p1, p2, p3]
    }
}
pub fn div_rect(num_a: u32, num_b: u32, scalar: u32) -> Vec<Rectangle> {
    // コード1.2
    let num_a = (num_a * scalar) as f32;
    let num_b = (num_b * scalar) as f32;

    let mut wd = num_b as f32;

    let mut xpos = 0.;
    let mut ypos = 0.;
    let mut itr = 0;
    let mut rects = vec![];
    while wd > 0. {
        itr += 1;
        if itr % 2 == 1 {
            while xpos + wd <= num_a {
                rects.push(Rectangle {
                    x: xpos,
                    y: ypos,
                    w: wd,
                    h: wd,
                });
                xpos += wd;
            }
            wd = num_a - xpos;
        } else {
            while ypos + wd <= num_b {
                rects.push(Rectangle {
                    x: xpos,
                    y: ypos,
                    w: wd,
                    h: wd,
                });
                ypos += wd;
            }
            wd = num_b - ypos;
        }
    }
    rects
}

#[cfg(test)]
mod tests {
    use crate::algos::*;
    #[test]
    fn test_euclid() {
        assert_eq!(euclidean_gcd(14803, 12707), 131)
    }
}
