/// 获取一条数据，返回 sql 语句
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mssql_quick::{msget, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// // 查寻 id 为 6 的数据
/// let sql = msget!("for_test", 6, "id,content as cc");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Item {
///     id: u64,
///     cc: Option<String>
/// }
/// let res: Vec<Item> = ms_run_vec(&mut client, sql).await.unwrap();
///
/// // 根据指定字段查寻 如：查寻 uid = 3 的数据
/// let sql = msget!("table", {"uid": 3});
/// # });
/// ```
///
#[macro_export]
macro_rules! msget {
    ($t:expr, {$k:tt: $v:expr} $(,$select:expr)?$(,)?) => {
        {
            fn _type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_table(tt: &str) -> &str {
                let t_list: Vec<&str> = tt.split_whitespace().collect();
                let table_change = t_list[t_list.len() - 1];
                table_change
            }
            // 将没有带上表名的字段，都重新命名为 主表字段  main_t_change是重命名后的
            fn _rename_field(field: &str, main_t_change: &str) -> String {
                let mut tmp_name = field.to_string();
                if !field.contains(".") {
                    let tmp = main_t_change.to_string() + "." + field;
                    tmp_name = tmp;
                }
                tmp_name
            }
            fn _get_select<T: Into<String> + std::fmt::Display>(s: T, main_table_change: &str) -> String {
                let mut tmp_select = String::from("");
                for v in s.to_string().split(",").collect::<Vec<&str>>().iter() {
                    let tmpv = v.trim();
                    tmp_select = tmp_select + _rename_field(tmpv, main_table_change).as_str() + ",";
                }
                tmp_select.pop();
                tmp_select
            }

            let keys = $k.to_string();
            let temp_v = $v;
            let v_type = _type_of(&temp_v);
            let values = match v_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&mssql_quick::method::method::Sql<&str>" |
                "&mssql_quick::method::method::Sql<alloc::string::String>" |
                "&mssql_quick::method::method::Sql<&alloc::string::String>" => {
                    temp_v.to_string().replace("Sql", "")
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                "&f32" | "&f64" | "&f128" | "&bool" => {
                    temp_v.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };
            let _table_change = get_table($t);
            let mut _select = "*";
            $(
                let tmp_s = _get_select($select, _table_change);
                _select = tmp_s.as_str();
            )?

            let sql = "SELECT TOP 1 ".to_string() + _select +
                " FROM " + $t +
                " WHERE " + keys.as_str() + "=" + values.as_str();

            sql
        }
    };
    ($t:expr, $v: expr $(,$select:expr)?$(,)?) => {
        {
            fn _type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_table(tt: &str) -> &str {
                let t_list: Vec<&str> = tt.split_whitespace().collect();
                let table_change = t_list[t_list.len() - 1];
                table_change
            }
            // 将没有带上表名的字段，都重新命名为 主表字段  main_t_change是重命名后的
            fn _rename_field(field: &str, main_t_change: &str) -> String {
                let mut tmp_name = field.to_string();
                if !field.contains(".") {
                    let tmp = main_t_change.to_string() + "." + field;
                    tmp_name = tmp;
                }
                tmp_name
            }
            fn _get_select<T: Into<String> + std::fmt::Display>(s: T, main_table_change: &str) -> String {
                let mut tmp_select = String::from("");
                for v in s.to_string().split(",").collect::<Vec<&str>>().iter() {
                    let tmpv = v.trim();
                    tmp_select = tmp_select + _rename_field(tmpv, main_table_change).as_str() + ",";
                }
                tmp_select.pop();
                tmp_select
            }

            let temp_v = $v;
            let v_type = _type_of(&temp_v);
            let values = match v_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&mssql_quick::method::method::Sql<&str>" |
                "&mssql_quick::method::method::Sql<alloc::string::String>" |
                "&mssql_quick::method::method::Sql<&alloc::string::String>" => {
                    temp_v.to_string().replace("Sql", "")
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                "&f32" | "&f64" | "&f128" | "&bool" => {
                    temp_v.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };
            let _table_change = get_table($t);
            let mut _select = "*";
            $(
                let tmp_s = _get_select($select, _table_change);
                _select = tmp_s.as_str();
            )?

            let sql = "SELECT TOP 1 ".to_string() + _select +
                " FROM " + $t +
                " WHERE id=" + values.as_str();

            sql
        }
    };
}
