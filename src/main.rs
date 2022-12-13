mod tests;

use std::{fs, io, panic, env};
use std::io::{BufRead, Write};
use std::path::{Path};
use telegraph_api_rs::{Telegraph, Request};
use rusqlite::{Connection};
use telegraph_api_rs::types::Node;


const USER_HOME_DIR: &str = ".images-to-telegraph";
const DB_FILE_NAME: &str = "images2telegraph.db";
const TELEGRAPH_BASE_URL: &str = "https://telegra.ph/";


// 账户
#[allow(dead_code)]
#[derive(Debug)]
struct TelegraphAccount {
    pub id: Option<i32>,
    pub short_name: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub access_token: Option<String>,
    pub auth_url: Option<String>,
    pub page_count: Option<i32>,
}

impl Clone for TelegraphAccount {
    fn clone(&self) -> Self {
        return TelegraphAccount {
            id: self.id,
            short_name: Option::from(self.short_name.as_ref().unwrap().to_string()),
            author_name: Option::from(self.author_name.as_ref().unwrap().to_string()),
            author_url: Option::from(self.author_url.as_ref().unwrap().to_string()),
            access_token: Option::from(self.access_token.as_ref().unwrap().to_string()),
            auth_url: Option::from(self.auth_url.as_ref().unwrap().to_string()),
            page_count: self.page_count,
        };
    }
}

impl TelegraphAccount {
    fn new_empty() -> Self {
        return TelegraphAccount {
            id: None,
            short_name: None,
            author_name: None,
            author_url: None,
            access_token: None,
            auth_url: None,
            page_count: None,
        };
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let images_path: &str;
    println!("{:?}", args);
    if args.len() <= 1 {
        println!("usage: images-to-telegram -d <images_folder>");
        return;
    } else if args.len() == 3 {
        let para = args.get(1).unwrap();
        if para != "-d"{
            println!("usage: images-to-telegram -d <images_folder>");
            return;
        }
        images_path = args.get(2).unwrap();
    }

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
        let conn = Connection::open(&db_file_path).unwrap();

        let account_sql = "create table  if not exists account(
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            short_name  TEXT NOT NULL,
            author_name TEXT NOT NULL,
            author_url TEXT NOT NULL,
            access_token TEXT NOT NULL,
            auth_url TEXT NOT NULL,
            page_count  INTEGER
        )";
        // 创建用户表
        // empty list of parameters.
        let success_flag = conn.execute(account_sql, ()).unwrap();
        if success_flag > 0 {
            println!("Create table account success!")
        }
        let post_sql = "create table if not exists post(
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            images_folder  TEXT NOT NULL,
            folder_hash TEXT NOT NULL,
            photo_counts INTEGER,
            access_url TEXT NOT NULL,
            update_time TEXT NOT NULL
        )";
        // 创建post表
        let success_flag = conn.execute(post_sql, ()).unwrap();
        if success_flag > 0 {
            println!("Create table post success!")
        }
    } else {
        //存在db 但是不存在表
        let conn = Connection::open(&db_file_path).unwrap();
        let tables_str = "select tbl_name from sqlite_master where tbl_name in ('account','post')";
        let mut sts = conn.prepare(tables_str).unwrap();
        if !sts.exists([]).unwrap() {
            let account_sql = "create table  if not exists account(
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            short_name  TEXT NOT NULL,
            author_name TEXT NOT NULL,
            author_url TEXT NOT NULL,
            access_token TEXT NOT NULL,
            auth_url TEXT NOT NULL,
            page_count  INTEGER
        )";
            // 创建用户表
            // empty list of parameters.
            let success_flag = conn.execute(account_sql, ()).unwrap();
            if success_flag > 0 {
                println!("Create table account success!")
            }
            let post_sql = "create table if not exists post(
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            images_folder  TEXT NOT NULL,
            folder_hash TEXT NOT NULL,
            photo_counts INTEGER,
            access_url TEXT NOT NULL,
            update_time TEXT NOT NULL
        )";
            // 创建post表
            let success_flag = conn.execute(post_sql, ()).unwrap();
            if success_flag > 0 {
                println!("Create table post success!")
            }
        }
    }
    // 有db文件是否已经创建了账户
    let conn = Connection::open(&db_file_path).unwrap();
    let account_query_str = "select * from account limit 1";
    let mut statement = conn.prepare(account_query_str).unwrap();

    let mut exist_account: TelegraphAccount = TelegraphAccount::new_empty();
    if !statement.exists([]).unwrap() {
        println!("No account in local db file!");
        // 获取用户输入创建账号
        let need_create_account = prompt("Do you want to create a new telegraph account? (yY/nN): ");
        match need_create_account {
            Some(value) => {
                if value.trim() == "y" {
                    // 没有账户创建账户
                    let telegraph = Telegraph::new();

                    let mut account = telegraph.create_account();
                    let tg_account = account.short_name("Fireinrain")
                        .author_name("Fireinrain with cosmos")
                        .author_url("https://fireinrain.github.io").send().unwrap();
                    println!("{:#?}", tg_account);
                    //存入本地db
                    let telegraph_account = TelegraphAccount {
                        id: None,
                        short_name: tg_account.short_name,
                        author_name: tg_account.author_name,
                        author_url: tg_account.author_url,
                        access_token: tg_account.access_token,
                        auth_url: tg_account.auth_url,
                        page_count: tg_account.page_count,
                    };
                    println!("{:#?}", telegraph_account);
                    exist_account = telegraph_account.clone();

                    let insert_sql = "insert into account(short_name,author_name,author_url,access_token,auth_url,page_count) values (?1,?2,?3,?4,?5,?6)";
                    let page_count = match telegraph_account.page_count {
                        None => { 0 }
                        Some(value) => { value }
                    };

                    let counts = conn.execute(insert_sql, (
                        &telegraph_account.short_name.unwrap(),
                        &telegraph_account.author_name.unwrap(),
                        &telegraph_account.author_url.unwrap(),
                        &telegraph_account.access_token.unwrap(),
                        &telegraph_account.auth_url.unwrap(),
                        page_count,
                    ));

                    match counts {
                        Ok(value) => {
                            println!("insert {} row to account success!", value);
                            println!("create telegraph account success!");
                        }
                        Err(e) => {
                            println!("{}", e);
                            println!("insert row to account failed!");
                        }
                    }
                } else {
                    println!("Create telegraph account cancelled!");
                }
            }
            None => {
                // println!("创建telegraph账户已取消！");
                // ignore
            }
        }
    } else {
        // 已经有账号
        let result_iter = statement.query_map([], |row| {
            Ok(TelegraphAccount {
                id: row.get(0)?,
                short_name: row.get(1)?,
                author_name: row.get(2)?,
                author_url: row.get(3)?,
                access_token: row.get(4)?,
                auth_url: row.get(5)?,
                page_count: row.get(6)?,
            })
        }).unwrap();
        for account in result_iter {
            exist_account = account.unwrap();
            println!("Found account in db {:#?}", exist_account);
            break;
        }
    }
    println!("use local db account for default...");
    // 上传新的post 到telegraph 并将浏览链接
    // 保存到db
    // println!("{:#?}", exist_account);
    let telegraph = Telegraph::new();
    let mut page = telegraph.create_page();

    let access_token = exist_account.access_token.unwrap();
    let mut vec = Vec::<Node>::new();
    vec.push(Node::String("a".to_string()));
    vec.push(Node::String("b".to_string()));
    let result = page.access_token(&access_token)
        .title("this is a test")
        .author_url("https://github.com/")
        .content(vec)
        .send().unwrap();

    let post_visit_url = TELEGRAPH_BASE_URL.to_string() + &result.path;
    println!("Create post success, visit url: {}", post_visit_url);

    // TODO need impliment;
    //保存访问链接
    //Hash 图片文件夹
    //设置图片post
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
