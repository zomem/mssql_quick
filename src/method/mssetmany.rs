///
/// 批量新增数据 ，返回 sql 语句。
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use mssql_quick::{mssetmany, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     title: String,
///     content: String,
///     price: f32,
///     total: Option<u32>,
///     uid: u32,
/// }
/// let vec_data = vec![
///     Item {
///         title: "mssetmany名字".to_string(),
///         content: "null".to_string(),
///         price: 32.23,
///         total: Some(12),
///         uid: 3
///     },
///     Item {
///         title: "mssetmany名字名字2".to_string(),
///         content: String::from(r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#),
///         price: 12.2,
///         total: None,
///         uid: 2
///     },
/// ];
/// let sql = mssetmany!("for_test", vec_data);
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
#[macro_export]
macro_rules! mssetmany {
    ($t:expr, $v: expr) => {{
        let mut field_name = " (".to_string();
        let mut value = "".to_string();
        for i in 0..$v.len() {
            let item_str = $crate::to_string(&$v[i]).unwrap();
            let o: $crate::Value = $crate::from_str(&item_str).unwrap();
            value = value + " (";
            for key in o.as_object().unwrap().keys() {
                if i == 0 {
                    field_name = field_name + &key + ",";
                }
                let temp_v = &o[key];
                if (temp_v.is_number()) {
                    value = value + temp_v.to_string().as_str() + ",";
                } else if temp_v.is_null() {
                    value = value + "NULL,";
                } else if temp_v.is_string() {
                    let t_v = temp_v.as_str().unwrap();
                    if t_v == "null" {
                        value = value + "NULL,";
                    } else {
                        let mut v_r = t_v.to_string();
                        v_r = v_r.replace("'", "''");
                        value = value + "N'" + &v_r + "',"
                    }
                }
            }
            if i == 0 {
                field_name.pop();
                field_name = field_name + ")";
            }
            value.pop();
            value = value + "),";
        }
        value.pop();

        let sql: String = "(INSERT INTO ".to_string()
            + $t
            + field_name.as_str()
            + " VALUES"
            + value.as_str()
            + ")";

        sql
    }};
}
