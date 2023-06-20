use gen::password;

mod gen;

fn main() {
    let pwd = password(16, &gen::DEFAULT_CHARSET);
    println!("{}", pwd);
}
