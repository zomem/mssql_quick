/// 新增数据 ，返回 sql 语句。
///
/// ```
/// # use mssql_quick::{msset, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// # let des_str = r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// let sql = msset!("users", {
///     "nickname": "张三",
///     "age": 3,
///     "content": "null",   // null 表示该字段为NULL
///     "des": des_str,
/// });
/// let set_res: Vec<MssqlQuickSet> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
#[macro_export]
macro_rules! msset {
    ($t:expr, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let mut keys = String::from("");
            let mut values = String::from("");
            $(
                keys = keys + $k + ",";
            )+
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                if temp_v.to_string().as_str() == "null" {
                    values = values + "NULL,";
                } else {
                    values = match v_type {
                        "&&str" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            values + "N'" + &v_r + "',"
                        },
                        "&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            values + "N'" + &v_r + "',"
                        },
                        "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            values + "N'" + &v_r + "',"
                        },
                        _ => {
                            values + temp_v.to_string().as_str() + ","
                        }
                    };
                }
            )+

            keys.pop();
            values.pop();

            let sql: String = "declare @id bigint; INSERT INTO ".to_string() + $t + " ( " + keys.as_str() + " ) "
                + " VALUES ( " + values.as_str() + " ) SET @id = scope_identity(); SELECT @id AS id ";

            sql
        }
    };
}
