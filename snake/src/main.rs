mod draw;

// 因为在项目生成时，rust只看到了 main.rs 这个crate，所以要在 main.rs 中 consts 声明为一个子模块
mod consts;

fn main() {
    println!("Hello, world!");

    for i in (0..3).rev() {
        println!("{}", i);
    }
}
