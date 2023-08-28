/// 批量新增数据 ，返回 sql 语句。
/// 下面示例中，user 为表名，，name、num 为字段名，，后面为新增的值。
///
/// ```
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     content: String,
///     total: u32,
/// }
/// let vec_data = vec![
///     Item {content: String::from("aaa"), total: 12},
///     Item {content: String::from("bb"), total: 1},
/// ];
/// let sql = mssetmany!("content", vec_data);
/// ```
#[macro_export]
macro_rules! mssetmany {
    ($t:expr, $v: expr) => {{
        // fn type_of<T>(_: T) -> &'static str {
        //     std::any::type_name::<T>()
        // }
        let mut field_name = " (".to_string();
        let mut value = "".to_string();
        for i in 0..$v.len() {
            let mut item_str = serde_json::to_string(&$v[i]).unwrap();
            item_str.pop();
            item_str.remove(0);
            item_str.push(',');
            item_str.push('"');
            item_str.insert(0, ',');
            // ",\"content\":\"aaa\",\"total\":12,\"uid\":3,\"des\":\"nn\",\""
            value = value + " (";
            let re2 = regex::Regex::new("\":(.*?),\"").unwrap();
            for cap2 in re2.captures_iter(item_str.as_str()) {
                let temp_v = &cap2[1];
                let mut value_cap;
                if temp_v == "null" {
                    value_cap = "NULL,".to_owned();
                } else {
                    value_cap = temp_v.to_string() + ",";
                    if let Some(c) = temp_v.chars().next() {
                        if c == '"' {
                            let mut v_r = temp_v.to_string();
                            v_r.remove(0);
                            v_r.pop();
                            v_r = v_r.replace("'", "''");
                            value_cap = "N'".to_owned() + &v_r + "',";
                        }
                    }
                }
                value = value + value_cap.as_str();
            }
            value.pop();
            value = value + "),";

            if i == 0 {
                let re = regex::Regex::new(",\"([0-9a-zA-Z_]+?)\":").unwrap();
                for cap in re.captures_iter(item_str.as_str()) {
                    field_name = field_name + &cap[1] + ",";
                }
                field_name.pop();
                field_name = field_name + ")";
            }
        }
        value.pop();

        let sql: String =
            "INSERT INTO ".to_string() + $t + field_name.as_str() + " VALUES" + value.as_str();

        sql
    }};
}
