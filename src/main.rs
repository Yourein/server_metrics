mod systeminfo;

fn main() {
    if !systeminfo::is_supported() {
        panic!{"This system is not supported! Exiting..."};
    }

    println!{"{:?}", systeminfo::fetch_info()};
}
