/// 1.单个条件，批量更新数据 ，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mssql_quick::{msupdatemany, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// # let des_str = r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     id: u64,
///     content: String,
///     total: Option<u32>,
/// }
/// let vec_data = vec![
///     Item {id: 7, content: String::from("批量更新11"), total: None},
///     Item {id: 8, content: des_str.to_string(), total: Some(10)},
/// ];
/// // 当前以 id 字段为查寻条件，更新 id 分别为7、8数据的content、total为对应的值。
/// let sql = msupdatemany!("for_test", "id", vec_data);
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
/// 2.多个条件，批量更新数据 ，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mssql_quick::{msupdatemany, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// # let des_str = r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// #[derive(Serialize, Deserialize)]
/// struct Item<'a> {
///     title: &'a str,
///     content: String,
///     total: u32,
/// }
/// let vec_data = vec![
///     Item {title: "a", content: String::from("aaa"), total: 32},
///     Item {title: "b", content: des_str.to_string(), total: 22},
/// ];
/// // 当前以 title && total 字段为查寻条件，"a" && 12 与 ”b“ && 1 的数据content为对应的值。
/// let sql = msupdatemany!("for_test", "title,total", vec_data);
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
///
/// 3.对特定字段进行原子性批量更新数据，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mssql_quick::{msupdatemany, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// # let des_str = r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// #[derive(Serialize, Deserialize)]
/// struct Item<'a> {
///     title: &'a str,
///     price: f32,
///     total: u32,
/// }
/// let vec_data = vec![
///     Item {title: "aa", price: 100., total: 1},
///     Item {title: "bb", price: 200., total: 1},
/// ];
/// // 需要行进 incr 更新的字段，用+号填写。
/// // 如下，表示以 title,total为查寻条件，price 字段要进行 incr 更新操作(注：price 不会作为查寻条件)。
/// let sql = msupdatemany!("for_test", "title,total,+price", vec_data);
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
#[macro_export]
macro_rules! msupdatemany {
    ($t:expr, $i:expr, $v: expr) => {{
        let i_info = $i.clone();
        let i_vec: Vec<String> = i_info
            .split(",")
            .into_iter()
            .map(|info| info.to_string())
            .collect();

        let mut incr_field: Vec<String> = vec![];
        let mut query_field: Vec<String> = vec![];
        for m in 0..i_vec.len() {
            if i_vec[m].contains("+") {
                incr_field.push(i_vec[m].clone())
            } else {
                query_field.push(i_vec[m].clone())
            }
        }
        // 中间生成的表名
        let table_upmj = $t.clone().to_owned() + "_upmj";
        let table = $t.clone().to_owned();

        let i_data = query_field
            .clone()
            .into_iter()
            .map(|x| format!(" {}.{} = {}.{} ", table, x, table_upmj, x))
            .collect::<Vec<String>>()
            .join("AND");

        let mut field_equl: Vec<String> = vec![];
        let mut select_vec: Vec<String> = vec![];

        for i in 0..$v.len() {
            let item_str = $crate::to_string(&$v[i]).unwrap();
            let o: $crate::Value = $crate::from_str(&item_str).unwrap();

            // SELECT  1 AS id, 11 AS code, 'nam' AS name, 44 AS book
            let mut field_list: Vec<&str> = vec![];
            let mut select_item: Vec<String> = vec![];

            for key in o.as_object().unwrap().keys() {
                if i == 0 {
                    field_list.push(&key);
                }

                let temp_v = &o[key];
                if (temp_v.is_number()) {
                    select_item.push(temp_v.to_string() + " AS " + &key);
                } else if temp_v.is_null() {
                    select_item.push("NULL".to_owned() + " AS " + &key);
                } else if temp_v.is_string() {
                    let t_v = temp_v.as_str().unwrap();
                    if t_v == "null" {
                        select_item.push("NULL".to_owned() + " AS " + &key);
                    } else {
                        let mut v_r = t_v.to_string();
                        v_r = v_r.replace("'", "''");
                        select_item.push("N'".to_owned() + &v_r + "'" + " AS " + &key);
                    }
                }
            }

            select_vec.push("SELECT ".to_string() + select_item.join(",").as_str());

            if i == 0 {
                field_equl = field_list
                    .iter()
                    .map(|x| {
                        if query_field.contains(&x.to_string()) {
                            return "".to_owned();
                        }
                        let mut is_incr = false;
                        for c in 0..incr_field.len() {
                            if incr_field[c].contains(x) {
                                is_incr = true;
                                break;
                            }
                        }
                        if is_incr {
                            table.clone()
                                + "."
                                + x
                                + " = "
                                + table.clone().as_str()
                                + "."
                                + x
                                + " + "
                                + table_upmj.as_str()
                                + "."
                                + x
                        } else {
                            table.clone() + "." + x + " = " + table_upmj.as_str() + "." + x
                        }
                    })
                    .filter(|o| o != &String::default())
                    .collect();
            }
        }

        let sql: String = "UPDATE ".to_string()
            + $t
            + " SET "
            + field_equl.join(", ").as_str()
            + " FROM "
            + $t
            + " INNER JOIN ("
            + select_vec.join(" UNION ").as_str()
            + ") AS "
            + table_upmj.as_str()
            + " ON"
            + i_data.as_str();

        sql
    }};
}
