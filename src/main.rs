use std::{fs, io, panic};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use telegraph_rs::telegraph::Telegraph;
use rusqlite::{Connection, Result};


const USER_HOME_DIR: &str = ".images-to-telegraph";
const DB_FILE_NAME: &str = "images2telegraph.db";

// 账户
type Account = telegraph_rs::types::Account;

fn main() -> Result<()> {
    // 找到用户目录
    let home_path = match home::home_dir() {
        None => {
            println!("Impossible to get your home dir!");
            None
        }
        Some(path) => {
            println!("Your home dir: {}", path.display());
            Some(path)
        }
    };

    let work_user_home = match home_path {
        None => { panic!("Can't get your home dir!") }
        Some(value) => { value.join(Path::new(USER_HOME_DIR)) }
    };

    // 没有就提示创建工作目录
    if !work_user_home.exists() {
        println!("Create your working directory: {}", work_user_home.display());
        fs::create_dir_all(&work_user_home).expect("can't create directory!");
    }
    println!("Working directory {}", work_user_home.display());
    // 有工作目录 判断是否有db文件
    let db_file_path = work_user_home.join(Path::new(DB_FILE_NAME));

    println!("Telegraph db file path: {}", db_file_path.display());

    if !db_file_path.exists() {
        let conn = Connection::open(&db_file_path)?;

        let account_sql = "create table  if not exists account(
            id    INTEGER PRIMARY KEY,
            short_name  TEXT NOT NULL,
            author_name TEXT NOT NULL,
            access_token TEXT NOT NULL,
            auth_url TEXT NOT NULL,
            page_count  INTEGER
        )";
        // 创建用户表
        // empty list of parameters.
        let success_flag = conn.execute(account_sql, ())?;
        if success_flag > 0 {
            println!("创建account表成功!")
        }
        let post_sql = "create table if not exists post(
            id    INTEGER PRIMARY KEY,
            images_folder  TEXT NOT NULL,
            photo_counts INTEGER,
            access_url TEXT NOT NULL,
            update_time TEXT NOT NULL
        )";
        // 创建post表
        let success_flag = conn.execute(post_sql, ())?;
        if success_flag > 0 {
            println!("创建post表成功!")
        }
    }
    // 有db文件是否已经创建了账户
    let conn = Connection::open(&db_file_path)?;
    let account_query_str = "select * from account limit 1";
    let mut statement = conn.prepare(account_query_str)?;
    if !statement.exists([])?{
        println!("当前数据库中暂无账号!");
        // 获取用户输入创建账号
        let need_create_account = prompt("是否需要创建telegraph账户? (yY/nN): ").unwrap().to_lowercase();
        let option_y = "y".to_string();
        match need_create_account == option_y{
            true => {
                let telegraph = Telegraph::new();

                let mut account = telegraph.create_account();
                let tg_account = account.short_name("Fireinrain")
                    .author_name("Fireinrain with cosmos")
                    .author_url("https://fireinrain.github.io").send().unwrap();
                println!("{:#?}",tg_account);

            },

            _ =>{
                println!("创建telegraph账户已取消！");
            }

        }


    }else {
        // 已经有账号
        let _result_iter = statement.query_map([], |row| {
            Ok(Account {
                short_name: row.get(0)?,
                author_name: row.get(1)?,
                author_url: row.get(2)?,
                access_token: row.get(3)?,
                auth_url: row.get(4)?,
                page_count: row.get(5)?
            })
        })?;
    }



    // 没有账户创建账户

    // 上传新的post 到telegraph 并将浏览链接
    // 保存到db



    // let mut page = telegraph.create_page();
    // page.author_name("Fireinrain").
    //     title("hello world")
    //     .send().expect("error");
    Ok(())
}

// 等待用户输入
fn prompt(message: &str) -> Option<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes()).unwrap();
    stdout.flush().unwrap();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    Some(line)
}
