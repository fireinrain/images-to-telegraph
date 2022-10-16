#[cfg(test)]
mod tests {
    use std::path::Path;
    use telegraph_rs::types::Account;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_insert_data_to_account() {
       let fake_account = Account{
           short_name: Some("xiaoqian".to_string()),
           author_name: Some("baba".to_string()),
           author_url: Some("https://github.com/".to_string()),
           access_token: Some("xxsadsadad".to_string()),
           auth_url: Some("https://github.com/".to_string()),
           page_count: Some(0)
       };
        use rusqlite::{Connection};
        let client = Connection::open(Path::new("/Users/sunrise/.images-to-telegraph/images2telegraph.db")).unwrap();
        let insert_sql = "insert into account (short_name, author_name, access_token,auth_url, page_count) values (?1, ?2, ?3, ?4, ?5)";

        let insert_account = client.execute(insert_sql, (
            &fake_account.short_name.unwrap(),
            &fake_account.author_name.unwrap(),
            &fake_account.access_token.unwrap(),
            &fake_account.author_url.unwrap(),
            &fake_account.page_count.unwrap()
        )).unwrap();

        assert_eq!(insert_account,1);

        let delete_sql = "delete from account where short_name = 'xiaoqian'";
        let delete_account = client.execute(delete_sql,()).unwrap();

        assert_eq!(delete_account,1);


    }
}


