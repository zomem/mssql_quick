/// 删除数据 ，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mssql_quick::{msdel, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// // 通过 id 删除（删除id为1066的数据）
/// let sql = msdel!("for_test", 1066);
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
///
/// // 通过指定字段的值，删除全部数据
/// // 删除 uid = 23 的数据 （注：如果有多条，会全部删除）
/// let sql = msdel!("for_test", {"uid": 23});
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
#[macro_export]
macro_rules! msdel {
    ($t:expr, {$k:tt: $v:expr}) => {{
        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let keys = $k.to_string();
        let temp_v = $v;
        let v_type = type_of(&temp_v);
        let values = match v_type {
            "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                let mut v_r = temp_v.to_string();
                v_r = v_r.replace("'", "''");
                "N'".to_string() + &v_r + "'"
            }
            "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" | "&i8" | "&i16" | "&i32"
            | "&i64" | "&i128" | "&isize" | "&f32" | "&f64" | "&f128" | "&bool" => {
                temp_v.to_string() + ""
            }
            _ => "".to_string(),
        };

        let sql: String =
            "DELETE FROM ".to_string() + $t + " WHERE " + keys.as_str() + "=" + values.as_str();

        sql
    }};

    ($t:expr, $v: expr) => {{
        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let temp_v = $v;
        let v_type = type_of(&temp_v);
        let values = match v_type {
            "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                let mut v_r = temp_v.to_string();
                v_r = v_r.replace("'", "''");
                "N'".to_string() + &v_r + "'"
            }
            "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" | "&i8" | "&i16" | "&i32"
            | "&i64" | "&i128" | "&isize" | "&f32" | "&f64" | "&f128" | "&bool" => {
                temp_v.to_string() + ""
            }
            _ => "".to_string(),
        };

        let sql: String = "DELETE FROM ".to_string() + $t + " WHERE id=" + values.as_str();

        sql
    }};
}
