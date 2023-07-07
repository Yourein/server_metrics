mod systeminfo;
mod discord;

fn main() {
    if !systeminfo::is_supported() {
        panic!{"This system is not supported! Exiting..."};
    }

    let res = discord::post_webhook(systeminfo::fetch_info());
    match res {
        Ok(()) => println!{"OK"},
        Err(e) => println!{"{}", e}
    }
}
