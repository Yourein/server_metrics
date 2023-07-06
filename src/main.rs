mod systeminfo;
mod discord;

fn main() {
    if !systeminfo::is_supported() {
        panic!{"This system is not supported! Exiting..."};
    }

    let res = discord::post_webhook();
    match res {
        Ok(()) => println!{"OK"},
        Err(e) => println!{"{}", e}
    }
}
