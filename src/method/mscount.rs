/// 计数 ，返回 sql 语句
///
/// 完整参数如下，参数同 find 类似。注意，参数可以省略，但顺序固定。
///
/// j*: 为 join 操作，【"字段1", "方法", "字段2"】
/// 方法有：`inner、left、left_outer、right、right_outer、full、full_outer、cross`
///
/// p*: 为查寻操作，【"字段", "方法", "参数"】
/// 方法有：`>、<、=、!=、<=、>=、like、in、not_in、is_null`
///
/// r: 为p的组合条件(必填)，如：`p0`、`p1 && (p0 || p2)`
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// use mssql_quick::{mscount, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickCount};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
///
/// let sql = mscount!("for_test", {});
/// let sql = mscount!("for_test", {
///     p0: ["price", ">", 100],
///     r: "p0",
/// });
/// let res_count: Vec<MssqlQuickCount> = ms_run_vec(&mut client, sql).await.unwrap();
///
/// // 其他用法
/// let sql = mscount!("for_test", {
///    p0: ["price", ">", 100],
///    r: "p0",
///    distinct: "title",
/// });
/// # });
/// ```
#[macro_export]
macro_rules! mscount {
    ($t:expr, {
        $(j0: [$jk0:tt, $jm0:tt, $jv0:expr],)?
        $(j1: [$jk1:tt, $jm1:tt, $jv1:expr],)?
        $(j2: [$jk2:tt, $jm2:tt, $jv2:expr],)?
        $(j3: [$jk3:tt, $jm3:tt, $jv3:expr],)?
        $(p0: [$k0:tt, $m0:tt, $v0:expr],)?
        $(p1: [$k1:tt, $m1:tt, $v1:expr],)?
        $(p2: [$k2:tt, $m2:tt, $v2:expr],)?
        $(p3: [$k3:tt, $m3:tt, $v3:expr],)?
        $(p4: [$k4:tt, $m4:tt, $v4:expr],)?
        $(p5: [$k5:tt, $m5:tt, $v5:expr],)?
        $(p6: [$k6:tt, $m6:tt, $v6:expr],)?
        $(p7: [$k7:tt, $m7:tt, $v7:expr],)?
        $(p8: [$k8:tt, $m8:tt, $v8:expr],)?
        $(p9: [$k9:tt, $m9:tt, $v9:expr],)?
        $(r: $r:expr,)?
        $(distinct: $distinct:expr,)?
    }) => {
        {
            use $crate::Regex;
            fn _type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            // 获取table  "users as u"
            fn get_table(tt: &str) -> &str {
                let t_list: Vec<&str> = tt.split_whitespace().collect();
                let table_change = t_list[t_list.len() - 1];
                table_change
            }
            // 获取table.field ”users as u.id“，如果只有 field 则table 为主table
            fn _get_table_f<'a>(tf: &'a str, main_t: &'a str) -> (String, String, String, bool) {
                let t_list: Vec<&str> = tf.split_whitespace().collect();
                let tf = t_list[0];
                let mut _tmp_t = "".to_string();  // talbe
                let mut _tmp_f = "".to_string();  // field
                if tf.contains("..") {
                    let tmp_vc_d: Vec<&str> = tf.split("..").collect();
                    let tmp_vc: Vec<&str> = tmp_vc_d[1].split(".").collect();
                     _tmp_t = format!("{}..{}", tmp_vc_d[0], tmp_vc[0]);
                     _tmp_f = tmp_vc[1].to_string();
                } else if tf.contains(".") {
                    let tmp_vc: Vec<&str> = tf.split(".").collect();
                    if tmp_vc.len() == 3 {
                         _tmp_t = format!("{}.{}", tmp_vc[0], tmp_vc[1]);
                         _tmp_f = tmp_vc[2].to_string();
                    } else {
                        _tmp_t = tmp_vc[0].to_string();
                        _tmp_f = tmp_vc[1].to_string();
                    }
                } else {
                    _tmp_t = main_t.to_string();
                    _tmp_f = tf.to_string();
                }
                let re_name_table;
                let table_change = if t_list.len() > 1 {
                    re_name_table = true;
                    t_list[t_list.len() - 1].to_string()
                } else {
                    re_name_table = false;
                    _tmp_t.clone()
                };
                (_tmp_t, table_change, _tmp_f, re_name_table)  // ("users as u", "u", id)
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
            fn _get_p_in(tmp_v: String, is_sql: bool) -> String {
                if is_sql {
                    tmp_v.to_string().replace("Sql", "")
                } else {
                    let tmp_vl: Vec<&str> = tmp_v.split(",").collect();
                    let mut tmp_vs: Vec<String> = vec![];
                    for t in tmp_vl.iter() {
                        let mut v_r = t.to_string();
                        v_r = v_r.replace("'", "''");
                        tmp_vs.push("N'".to_string() + &v_r + "'");
                    }
                    "(".to_string() + tmp_vs.join(",").as_str() + ")"
                }
            }
            fn _get_p(k: &str, m: &str, v: &str, vty: &str, main_table_change: &str) -> String {
                let mut tmp_v = v.to_string();
                let mut is_sql = false;
                if m == "in" || m == "not_in" || m == "is_null" {
                    if vty == "&mssql_quick::method::method::Sql<&str>" ||
                    vty == "&mssql_quick::method::method::Sql<alloc::string::String>" ||
                    vty == "&mssql_quick::method::method::Sql<&alloc::string::String>" {
                        is_sql = true;
                    }
                } else {
                    tmp_v = match vty {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "'"
                        },
                        "&mssql_quick::method::method::Sql<&str>" |
                        "&mssql_quick::method::method::Sql<alloc::string::String>" |
                        "&mssql_quick::method::method::Sql<&alloc::string::String>" => {
                            v.to_string().replace("Sql", "")
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                        "&f32" | "&f64" | "&f128" | "&bool" => {
                            v.to_string() + ""
                        },
                        "&&u8" | "&&u16" | "&&u32" | "&&u64" | "&&u128" | "&&usize" |
                        "&&i8" | "&&i16" | "&&i32" | "&&i64" | "&&i128" | "&&isize" |
                        "&&f32" | "&&f64" | "&&f128" | "&&bool" => {
                            v.to_string() + ""
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let k_re = _rename_field(k, main_table_change);
                let p = match m {
                    ">" => k_re + " > " + tmp_v.as_str(),
                    "<" => k_re + " < " + tmp_v.as_str(),
                    "=" => k_re + " = " + tmp_v.as_str(),
                    ">=" => k_re + " >= " + tmp_v.as_str(),
                    "<=" => k_re + " <= " + tmp_v.as_str(),
                    "!=" => k_re + " != " + tmp_v.as_str(),
                    "like" => k_re + " LIKE " + tmp_v.as_str(),
                    "in" => k_re + " IN " + _get_p_in(tmp_v, is_sql).as_str(),
                    "not_in" => k_re + " NOT IN " + _get_p_in(tmp_v, is_sql).as_str(),
                    "is_null" => {
                        let is_null = if tmp_v == "true" {"NULL"} else {"NOT NULL"};
                        k_re + " is " + is_null
                    },
                    _ => "".to_string(),
                };
                p
            }

            fn _get_j(k: &str, m: &str, v: &str, t: &str) -> String {
                let (_, k_table_re, k_field, _) = _get_table_f(k, t);
                let (v_table, v_table_re, v_field, v_rename) = _get_table_f(v, t);
                let k_table_re = k_table_re.as_str();
                let k_field = k_field.as_str();
                let v_table = v_table.as_str();
                let v_table_re = v_table_re.as_str();
                let v_field = v_field.as_str();
                let as_vtable = if v_rename {
                    " AS ".to_string() + v_table_re
                } else {
                    "".to_string()
                };
                let j_string = match m {
                    "inner" => " INNER JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "left" => " LEFT JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "left_outer" => " LEFT OUTER JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "right" => " RIGHT JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "right_outer" => " RIGHT OUTER JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "full" => " FULL JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "full_outer" => " FULL OUTER JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    "cross" => " CROSS JOIN ".to_string() + v_table + as_vtable.as_str() + " ON " + k_table_re + "." + k_field + " = " + v_table_re + "." + v_field,
                    _ => "".to_string()
                };
                j_string
            }


            let _table_change = get_table($t);

            let mut _join = String::from("");
            {
                $(
                    _join = _join + _get_j($jk0, $jm0, $jv0, $t).as_str();
                )?
            }
            {
                $(
                    _join = _join + _get_j($jk1, $jm1, $jv1, $t).as_str();
                )?
            }
            {
                $(
                    _join = _join + _get_j($jk2, $jm2, $jv2, $t).as_str();
                )?
            }
            {
                $(
                    _join = _join + _get_j($jk3, $jm3, $jv3, $t).as_str();
                )?
            }


            let mut hash_p: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            {
                $(
                    let tm = $v0;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p0".to_string(), _get_p($k0, $m0, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v1;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p1".to_string(), _get_p($k1, $m1, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v2;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p2".to_string(), _get_p($k2, $m2, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v3;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p3".to_string(), _get_p($k3, $m3, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v4;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p4".to_string(), _get_p($k4, $m4, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v5;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p5".to_string(), _get_p($k5, $m5, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v6;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p6".to_string(), _get_p($k6, $m6, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v7;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p7".to_string(), _get_p($k7, $m7, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v8;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p8".to_string(), _get_p($k8, $m8, t.as_str(), vt, _table_change));
                )?
            }
            {
                $(
                    let tm = $v9;
                    let vt = _type_of(&tm);
                    let t = tm.to_string();
                    hash_p.insert("p9".to_string(), _get_p($k9, $m9, t.as_str(), vt, _table_change));
                )?
            }


            let mut where_r = String::from("");
            let mut _r = String::from("");
            let mut _list: Vec<&str> = vec![];
            $(
                _r = $r.split_whitespace().collect();
                let re1 = Regex::new(r"(\()|(\))").unwrap();
                _r = re1.replace_all(_r.as_str(), "#$1$2#").to_string();
                _list = _r.split("#").collect();
            )?

            if (_list.len() > 0) {
                let mut stack: Vec<String> = vec![];
                let mut top_brackets = String::from("");
                for (index, item) in _list.iter().enumerate() {
                    if *item == ")" {
                        // 出栈
                        let mut stack_top = if let Some(s) = stack.pop() {s.to_string()} else {"".to_string()};
                        loop {
                            if stack_top != "(" {
                                top_brackets = stack_top + top_brackets.as_str();
                                stack_top = if let Some(s) = stack.pop() {s.to_string()} else {break;};
                                continue;
                            } else {
                                break;
                            }
                        }
                        //进行and,or
                        let re2 = Regex::new(r"(&&)|(\|\|)").unwrap();
                        let top_re2 = re2.replace_all(top_brackets.as_str(), "#$1$2#");
                        let temp_arr: Vec<&str> = top_re2.split("#").collect();
                        // [p9, &&, p8, ||, p32]

                        let mut temp_qq = if let Some(p_x) = hash_p.get(temp_arr[0]) {p_x.to_string()} else {"".to_string()};
                        let mut n = 0;

                        loop {
                            if n < temp_arr.len() - 1 {
                                let tmp = if let Some(p_x) = hash_p.get(temp_arr[n + 2]) {p_x} else {""};
                                if temp_arr[n + 1] == "&&" {
                                    temp_qq = "(".to_string() + temp_qq.as_str() + " AND " + tmp + ")";
                                }
                                if temp_arr[n + 1] == "||" {
                                    temp_qq = "(".to_string() + temp_qq.as_str() + " OR " + tmp + ")";
                                }
                                n += 2;
                                continue;
                            } else {
                                break;
                            }
                        }
                        let tmp_pp_name = ("pp".to_string() + index.to_string().as_str()).to_owned();
                        let tmp_pp_n = tmp_pp_name.clone();
                        hash_p.insert(tmp_pp_name, temp_qq);
                        top_brackets = String::from("");
                        stack.push(tmp_pp_n);
                        continue;
                    } else {
                        // 入栈
                        stack.push(item.to_string());
                    }
                }
                let temp_arr_str2 = String::from_iter(stack);
                let re3 = Regex::new(r"(&&)|(\|\|)").unwrap();
                let top_re3 = re3.replace_all(temp_arr_str2.as_str(), "#$1$2#");
                let temp_arr2: Vec<&str> = top_re3.split("#").collect();

                let mut qq_all = if let Some(p_x) = hash_p.get(temp_arr2[0]) {p_x.to_string()} else {"".to_string()};
                let mut n_all = 0;
                loop {
                    if n_all < temp_arr2.len() - 1 {
                        let tmp = if let Some(p_x) = hash_p.get(temp_arr2[n_all + 2]) {p_x} else {""};
                        if temp_arr2[n_all + 1] == "&&" {
                            qq_all = "(".to_string() + qq_all.as_str() + " AND " + tmp + ")"
                        }
                        if temp_arr2[n_all + 1] == "||" {
                            qq_all = "(".to_string() + qq_all.as_str() + " OR " + tmp + ")"
                        }
                        n_all += 2;
                        continue;
                    } else {
                        break;
                    }
                }

                where_r = " WHERE ".to_string() + qq_all.as_str();
            }

            let mut _distinct = String::from("*");
            $(
                _distinct = format!("DISTINCT {}", $distinct);
            )?

            let sql = "SELECT count(".to_string() + _distinct.as_str() + ") as mssql_quick_count" +
                " FROM " + $t +
                _join.as_str() +
                where_r.as_str();

            sql
        }
    };
}
