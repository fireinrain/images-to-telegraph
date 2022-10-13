use telegraph_rs::telegraph::Telegraph;

fn main() {
    let telegraph = Telegraph::new();
    let mut account = telegraph.create_account();
    let tg_account = account.short_name("Fireinrain")
        .author_name("Fireinrain with cosmos")
        .author_url("https://fireinrain.github.io").send().unwrap();
    println!("{:#?}",tg_account)

    // let mut page = telegraph.create_page();
    // page.author_name("Fireinrain").
    //     title("hello world")
    //     .send().expect("error");
    
}
