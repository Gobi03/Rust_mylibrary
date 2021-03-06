#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle {
    leftup: Coord,
    rightdown: Coord,
}

impl Rectangle {
    fn new(leftup: Coord, rightdown: Coord) -> Self {
        Rectangle { leftup, rightdown }
    }

    fn calc_area(&self) -> isize {
        (self.rightdown.x - self.leftup.x) * (self.rightdown.y - self.leftup.y)
    }

    fn in_field(&self) -> bool {
        self.leftup.x >= 0
            && self.leftup.y >= 0
            && self.rightdown.x < SIDE as isize
            && self.rightdown.y < SIDE as isize
    }

    fn does_include_point(&self, point: &Coord) -> bool {
        let &Coord { x, y } = point;
        self.leftup.x <= x && x <= self.rightdown.x && self.leftup.y <= y && y <= self.rightdown.y
    }

    fn does_include_rect(&self, that: &Rectangle) -> bool {
        let in_x_overwrapped = self.leftup.x < that.rightdown.x && self.rightdown.x > that.leftup.x;
        let in_y_overwrapped = self.leftup.y < that.rightdown.y && self.rightdown.y > that.leftup.y;
        in_x_overwrapped && in_y_overwrapped
    }
}
