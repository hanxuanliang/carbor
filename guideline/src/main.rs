use std::fmt;

#[derive(Debug)]
struct Obj {
    width: u32,
    height: u32,
}

impl Obj {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Obj {
        Obj { width, height }
    }

    fn show(&self) {
        println!("{} * {}, area = {}", self.width, self.height, self.area())
    }

    // 需要在 struct 上，引用 Debug 宏
    fn debug_show(&self) {
        println!("{:#?}", self)
    }

    fn print_show(&self) {
        println!("{}", self)
    }
}

// 实现一个 Display 的 trait
impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    let o1 = Obj {
        width: 35,
        height: 55,
    };

    let o2 = Obj::new(44, 35);

    o1.show();
    o2.show();

    println!("===============");
    o1.debug_show();
    o2.debug_show();

    println!("===============");
    o1.print_show();
    o2.print_show();
}
