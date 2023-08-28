/// 新增数据 ，返回 sql 语句。
/// 下面示例中，user 为表名，，name、num 为字段名，，后面为新增的值。
/// ```
/// let sql = msset!("users", {
///     "name": &string_t,
///     "num": 882,
///     "content": "null",   // null 表示该字段为NULL
/// });
/// ms_run_vec(&mut client, sql).await.unwrap();
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
