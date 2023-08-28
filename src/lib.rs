mod method;
pub use method::*;

#[cfg(test)]
mod tests {
    use crate::{
        ms_run_vec, mscount, msdel, msfind, msget, msset, mssetmany, msupdate, msupdatemany,
        EncryptionLevel, MssqlQuick, MssqlQuickSet,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MsssqlTest {
        id: u32,
        title: String,
        content: Option<String>,
        price: f32,
        total: u32,
        uid: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MsssqlTestAdd {
        title: String,
        content: Option<String>,
        price: f32,
        uid: u32,
    }
    const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";

    #[tokio::test]
    async fn is_msset_ok() {
        let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
            .await
            .unwrap()
            .client;

        let sqlset = msset!("for_test", {
            "title": "这是一31113标题",
            "content": r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#,
            "price": 776,
            "total": 122,
            "uid": 23
        });

        println!("set sql语句：{}", sqlset);

        // let res: Vec<MssqlQuickSet> = ms_run_vec(&mut client, sqlset).await.unwrap();
        // println!("set返回：{:?}", res);
        // println!("set返回：{:?}", res[0].id);

        let sql2 = msset!("for_test", {
            "title": "内容题目",
            "content": "null",
            "price": 776,
            "total": 122,
            "uid": 23
        });
        // let res: Vec<MssqlQuickSet> = ms_run_vec(&mut client, sql2).await.unwrap();
        println!("set返回：{}", sql2);

        // 批量更新
        let slq_m = mssetmany!(
            "for_test",
            vec![
                MsssqlTestAdd {
                    title: "名字1".to_owned(),
                    content: None,
                    price: 12.3,
                    uid: 50
                },
                MsssqlTestAdd {
                    title: "名字2".to_owned(),
                    content: Some("内容。。。".to_owned()),
                    price: 32.3,
                    uid: 50
                },
                MsssqlTestAdd {
                    title: "名字3".to_owned(),
                    content: Some("2389)d(ŐдŐ๑)🍉 .',".to_owned()),
                    price: 32.3,
                    uid: 50
                },
                MsssqlTestAdd {
                    title: "名字4".to_owned(),
                    content: None,
                    price: 32.3,
                    uid: 50
                },
            ]
        );

        println!(" \n set_many ;:: sql ::::  {} \n", slq_m);
        // let res: Vec<serde_json::Value> = ms_run_vec(&mut client, slq_m).await.unwrap();
        // println!("set_many返回：{:?}", res);

        assert!(true)
    }

    #[tokio::test]
    async fn is_msget_ok() {
        let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
            .await
            .unwrap()
            .client;

        let sql = msget!("for_test", 1080, "*");
        println!("\nmsget sql 返回：{}", sql);
        let sql2 = msget!("for_test", {"uid": 7}, "*");
        println!("msget sql2 返回：{}", sql2);
        let sql3 = msget!("for_test", 2, "id, title, content as c, uid, price");
        println!("msget sql3 返回：{}", sql3);
        let sql4 = msget!("for_test", {"title": "内容题目"}, "id, title, content as c, uid, price");
        println!("msget sql4 返回：{}", sql4);
        let sql5 = msget!("for_test", {
            "content": r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#
        }, "id, content as c, uid, price");
        println!("msget sql5 返回：{}", sql5);

        #[derive(Serialize, Deserialize, Debug)]
        pub struct MsssqlChange {
            id: u32,
            title: String,
            c: Option<String>,
            price: f32,
            uid: u32,
        }
        let data: Vec<MsssqlChange> = ms_run_vec(&mut client, sql4).await.unwrap();
        println!("msget data 结果：{:?}", data);

        assert!(true)
    }

    #[tokio::test]
    async fn is_msfind_ok() {
        // let res2: Vec<MsssqlTest> =
        //     ms_run_vec(&mut client, String::from("SELECT top 8 * FROM for_test;"))
        //         .await
        //         .unwrap();
        // println!("res2  \n   {:#?} \n", res2);
        assert!(true)
    }

    #[tokio::test]
    async fn is_del_ok() {
        let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
            .await
            .unwrap()
            .client;

        let sql = msdel!("for_test", 1096);
        let sql = msdel!("for_test", {"content": "🍉"});

        let res: Vec<serde_json::Value> = ms_run_vec(&mut client, sql).await.unwrap();
        println!("删除：{:?}", res);

        assert!(true)
    }

    #[tokio::test]
    async fn is_update_ok() {
        let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
            .await
            .unwrap()
            .client;

        let sql = msupdate!("for_test", {"uid": 23}, {
            "title": "标题",
            "price": 199.999,
            "content": "(ŐдŐ๑)🍉 .',"
        });
        let sql = msupdate!("for_test", {"uid": 23}, {
            "title": ["set", "9992ad"],
            "total": ["incr", 10],
        });
        println!("更新返回sql：{}", sql);
        let up_res: Vec<serde_json::Value> = ms_run_vec(&mut client, sql).await.unwrap();
        println!("更新返回：{:?}", up_res);

        // 批量更新
        let ups_sql = msupdatemany!(
            "for_test",
            "uid,+price",
            vec![
                MsssqlTestAdd {
                    title: "名44444字2aaa".to_owned(),
                    content: Some("内77777容。。。".to_owned()),
                    price: 2.,
                    uid: 50
                },
                MsssqlTestAdd {
                    title: "名字333".to_owned(),
                    content: Some("2389888888888888🍉🍉)d(ŐдŐ๑)🍉 .',".to_owned()),
                    price: 100.,
                    uid: 51
                },
            ]
        );

        println!("\n\n 批量更新ups： {} \n\n", ups_sql);
        let up_res2: Vec<serde_json::Value> = ms_run_vec(&mut client, ups_sql).await.unwrap();
        println!(" {:?} \n\n", up_res2);
        assert!(true)
    }

    #[tokio::test]
    async fn is_find_ok() {
        let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
            .await
            .unwrap()
            .client;

        let sql = msfind!("for_test", {
            j0: ["uid", "inner", "users.id"],
            p0: ["uid", ">", 0],
            r: "p0",
            page: 1,
            limit: 10,
            order_by: "-price",
            select: "content, price, users.nickname",
        });
        println!("\n\n 查寻sql： {}  \n", sql);

        let sql = mscount!("for_test", {
            p0: ["uid", ">", 20],
            r: "p0",
        });

        let res: Vec<serde_json::Value> = ms_run_vec(&mut client, sql).await.unwrap();
        println!("\n\n 查寻返回：{:?}", res);

        assert!(true)
    }
}
