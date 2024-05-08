mod method;
pub use method::*;

pub use tiberius::{error::Error, Client, ColumnType, Config};
pub use tokio::net::TcpStream;
pub use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

#[cfg(test)]
mod tests {
    use crate::{mscount, msdelmany, msfind, msupdatemany, EncryptionLevel, MssqlQuick};
    use serde::{Deserialize, Serialize};

    const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";

    // #[derive(Serialize, Deserialize, Debug)]
    // struct Item {
    //     title: String,
    //     content: String,
    //     price: f32,
    //     total: u32,
    //     uid: u32,
    // }
    #[derive(Serialize, Deserialize)]
    struct Item {
        id: u64,
        content: String,
        total: u32,
    }

    #[tokio::test]
    async fn is_msset_ok() {
        let mut _client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
            .await
            .unwrap()
            .client;

        // 事务临时解决办法：
        // client.simple_query("BEGIN TRAN").await?;
        // // deal with your business, do not panic or error out
        // match result {
        //   Ok(_) => client.simple_query("COMMIT").await?,
        //   Err(_) => client.simple_query("ROLLBACK").await?,
        // }

        let vec_data = vec![
            Item {
                id: 7,
                content: String::from("批量更新1"),
                total: 0,
            },
            Item {
                id: 8,
                content: String::from("批量更新2"),
                total: 0,
            },
        ];

        // 当前以 id 字段为查寻条件，更新 id 分别为7、8数据的content、total为对应的值。
        let sql = msupdatemany!("for_test", "id", vec_data);
        println!("33333::::::: {}", sql);

        let _sql = msfind!("feedback as fb", {
            j0: ["uid", "inner", "users.id"],
            j1: ["uid", "inner", "users as u2.id"], // 对表重命名
            j2: ["book_id", "left", "book.id"],
            j3: ["book.uid", "right", "users.id"],
            p0: ["num", ">", 0],
            p1: ["d", "=", "这是的"],
            p2: ["users.user_niae", "like", "%aa%"],
            p3: ["ppp", "is_null", true],
            p4: ["u2.price", ">", 1],
            p5: ["u2.price", "like", "aa%"],
            p6: ["u2.price", "in", "zzz,nnn"],
            p7: ["u2.price", "not_in", "zm"],
            p8: ["f", "=", "32"],
            p9: ["u2.price", "is_null", true],
            r: "p8 && (p0 || p3) && (p1 && (p2 || p4))",  // 为p的组合规则
            page: 3,  // 第几页
            limit: 5, // 每页数量
            order_by: "-created_at,   time, -users.updated_at", // 排序
            select: "id, name,   avatar_url as aurl,users.c, u2.name", // 字段选择
        });

        let des_str = r#"#@!@$$^&^%&&#\\,abc,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',ddd"#;
        // let sql = msfind!("for_test", {
        //     p0: ["content", "=", des_str],
        //     r: "p0",
        //     select: "SUM(age)",
        //     group: "age",
        //     have: "age > 0",
        //     group_order_by: "-age",
        // });
        // let sql = msfind!("for_test", {
        //     p0: ["content", "=", "abc"],
        //     r: "p0",
        //     select: "DISTINCT name",
        // });

        // let sql = msget!("for_test", 6, "id,content as cc");
        // let sql = msget!("for_test", {"uid": 3}, "*");
        //
        let _sql = mscount!("for_test", {});
        let sql = msdelmany!("for_test", {
            p0: ["content", "in", des_str],
            r: "p0",  // 为p的组合规则
        });
        println!("22322222::::::: {}", sql);

        // let vec_data = vec![
        //     Item {
        //         title: "名字".to_string(),
        //         content: "null".to_string(),
        //         price: 32.23,
        //         total: 12,
        //         uid: 3,
        //     },
        //     Item {
        //         title: "名字2".to_string(),
        //         content: String::from(
        //             r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#,
        //         ),
        //         price: 12.2,
        //         total: 1,
        //         uid: 2,
        //     },
        // ];
        // let sql = mssetmany!("for_test", vec_data);
        // println!("mmmsql::::::: {}", sql);
        // let des_str =
        //     r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
        // let sql = msset!("users", {
        //     "nickname": "张三",
        //     "age": 3,
        //     "content": "null",   // null 表示该字段为NULL
        //     "des": des_str,
        // });
        // println!("ssssql::::::: {}", sql);

        assert!(true)
    }
}
