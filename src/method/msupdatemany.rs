/// 1.单个条件，批量更新数据 ，返回 sql 语句。
/// ```
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     id: u64,
///     content: String,
///     total: u32,
/// }
/// let vec_data = vec![
///     Item {id: 1, content: String::from("aaa"), total: 12},
///     Item {id: 2, content: String::from("bb"), total: 1},
/// ];
/// let sql = msupdatemany!("content", "id", vec_data);
/// // 当前以 id 字段为查寻条件，更新 id 分别为 1、2 的数据的content、total为对应的值。
/// ```
///
///
/// 2.多个条件，更新数据，返回 sql 语句。
/// ```
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     name: String,
///     content: String,
///     total: u32,
/// }
/// let vec_data = vec![
///     Item {name: "a", content: String::from("aaa"), total: 12},
///     Item {name: "b", content: String::from("bb"), total: 1},
/// ];
/// let sql = msyupdatemany!("content", "name,total", vec_data);
/// // 当前以 name && total 字段为查寻条件，更新 name 和 total 分别为 "a" && 12 与 ”b“ && 1 的数据的content为对应的值。
/// ```
/// 3.对某个字段进行原子性更新，返回 sql 语句。
/// ```
/// // 要行进 incr 的更新的字段，用+号填写。
/// // 如下，表示以name,total为查寻条件，price字段要进行incr更新操作(price 不会作为查寻条件)。
/// let sql = msupdatemany!("content", "name,total,+price", vec_data);
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
            .into_iter()
            .map(|x| format!(" {}.{} = {}.{} ", table, x, table_upmj, x))
            .collect::<Vec<String>>()
            .join("AND");

        let mut field_equl: Vec<String> = vec![];
        let mut select_vec: Vec<String> = vec![];

        for i in 0..$v.len() {
            let mut item_str = serde_json::to_string(&$v[i]).unwrap();
            item_str.pop();
            item_str.remove(0);
            item_str.push(',');
            item_str.push('"');
            item_str.insert(0, ',');
            // ",\"content\":\"aaa\",\"total\":12,\"uid\":3,\"des\":\"nn\",\""
            // SELECT  1 AS id, 11 AS code, 'nam' AS name, 44 AS book
            let mut field_list: Vec<String> = vec![];
            let mut select_item: Vec<String> = vec![];

            let re = regex::Regex::new(",\"([0-9a-zA-Z_]+?)\":").unwrap();
            for cap in re.captures_iter(item_str.as_str()) {
                field_list.push((&cap[1]).to_string());
            }

            let re2 = regex::Regex::new("\":(.*?),\"").unwrap();
            let mut n = 0;
            for cap2 in re2.captures_iter(item_str.as_str()) {
                let temp_v = &cap2[1];
                let mut value_cap;
                if temp_v == "null" {
                    value_cap = "NULL".to_owned();
                } else {
                    value_cap = temp_v.to_string();
                    if let Some(c) = temp_v.chars().next() {
                        if c == '"' {
                            let mut v_r = temp_v.to_string();
                            v_r.remove(0);
                            v_r.pop();
                            v_r = v_r.replace("'", "''");
                            value_cap = "N'".to_owned() + &v_r + "'";
                        }
                    }
                }

                select_item.push((&value_cap).to_string() + " AS " + field_list[n].as_str());
                n = n + 1;
            }

            select_vec.push("SELECT ".to_string() + select_item.join(",").as_str());

            if i == 0 {
                field_equl = field_list
                    .iter()
                    .map(|x| {
                        let mut is_incr = false;
                        for c in 0..incr_field.len() {
                            if incr_field[c].contains(x.as_str()) {
                                is_incr = true;
                                break;
                            }
                        }
                        if is_incr {
                            table.clone()
                                + "."
                                + x.as_str()
                                + " = "
                                + table.clone().as_str()
                                + "."
                                + x.as_str()
                                + " + "
                                + table_upmj.as_str()
                                + "."
                                + x.as_str()
                        } else {
                            table.clone()
                                + "."
                                + x.as_str()
                                + " = "
                                + table_upmj.as_str()
                                + "."
                                + x.as_str()
                        }
                    })
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
