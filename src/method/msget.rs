/// 获取一条数据，一定要记录传`select`的字段且不能为*
///
/// ```
/// 根据 id 查寻
/// // 查寻 id =3 3 的数据
/// let sql1 = msget!("feedback", 33, "id,content as cc");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Feedback {
///     id: u64,
///     cc: String
/// }
/// let res_get: Vec<Feedback> = ms_run_vec(&mut client, sql1).await.unwrap();
///
///
/// 根据指定字段查寻
/// // 查寻 uid = 32 的数据
/// msget!("table", {"uid": 32}, "*")
///
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
            fn _get_select(s: &str, main_table_change: &str) -> String {
                let mut tmp_select = String::from("");
                for v in s.split(",").collect::<Vec<&str>>().iter() {
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
                "&&str" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&alloc::string::String" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&&alloc::string::String" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                _ => {
                    temp_v.to_string() + ""
                }
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
            fn _get_select(s: &str, main_table_change: &str) -> String {
                let mut tmp_select = String::from("");
                for v in s.split(",").collect::<Vec<&str>>().iter() {
                    let tmpv = v.trim();
                    tmp_select = tmp_select + _rename_field(tmpv, main_table_change).as_str() + ",";
                }
                tmp_select.pop();
                tmp_select
            }

            let temp_v = $v;
            let v_type = _type_of(&temp_v);
            let values = match v_type {
                "&&str" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&alloc::string::String" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&&alloc::string::String" => {
                    let mut v_r = temp_v.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                _ => {
                    temp_v.to_string() + ""
                }
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
