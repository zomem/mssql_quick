/// 1.通过id，更新数据 ，返回 sql 语句。
/// ```
/// # use mssql_quick::{msupdate, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// # let des_str = r#"m'y,,a#@!@$$^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// let sql = msupdate!("for_test", 5, {
///     "title": "更新标题",
///     "uid": 6,
///     "content": des_str,
/// });  // id = 5
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
///
/// // 原子更新，(如果使用[字段，值]的方式，都所有都需要使用这种形式)
/// let sql = msupdate!("for_test", 6, {
///     "title": ["set", "价格减2"],  // set 修改操作
///     "price": ["incr", -2],   // incr 原子性加减
///     "content": ["unset", ""],   // unset 清空值
/// });
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
///
/// 2.通过指定字段的值，更新数据 ，返回 sql 语句。
/// ```
/// # use mssql_quick::{msupdate, ms_run_vec, MssqlQuick, EncryptionLevel, MssqlQuickSet};
/// # const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laFidia32FAEE534DFa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";
/// # tokio_test::block_on(async {
/// # let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported).await.unwrap().client;
/// let sql = msupdate!("for_test", {"uid": 5}, {"title": "更新了uid为5的数据"}); // 更新 uid = 5 的第一条数据
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
///
/// // 原子性更新
/// let sql = msupdate!("for_test", {"uid": 5}, {"total": ["incr", 1]});
/// let _: Vec<()> = ms_run_vec(&mut client, sql).await.unwrap();
/// # });
/// ```
///
#[macro_export]
macro_rules! msupdate {
    ($t:expr, {$ik:tt: $iv:expr}, {$($k:tt: [$m:tt, $v:expr]),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let tmp_ik = $ik.to_string();
            let i_data = $iv;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }

                let tmp_s = match $m {
                    "set" => $k.to_string() + "=" + value.as_str(),
                    "incr" => {
                        let mut op = "+";
                        let first = &value.as_str()[0..1];
                        if first == "-" {
                            op = ""
                        }
                        $k.to_string() + "=" + $k + op + value.as_str()
                    },
                    "unset" => $k.to_string() + "=NULL,",
                    _ => $k.to_string() + "=" + value.as_str(),
                };
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str() + ")";

            sql
        }
    };

    ($t:expr, {$ik:tt: $iv:expr}, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let tmp_ik = $ik.to_string();
            let i_data = $iv;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let tmp_s = $k.to_string() + "=" + value.as_str();
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str() + ")";

            sql
        }
    };

    ($t:expr, $i:expr, {$($k:tt: [$m:tt, $v:expr]),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let i_data = $i;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let tmp_s = match $m {
                    "set" => $k.to_string() + "=" + value.as_str(),
                    "incr" => {
                        let mut op = "+";
                        let first = &value.as_str()[0..1];
                        if first == "-" {
                            op = ""
                        }
                        $k.to_string() + "=" + $k + op + value.as_str()
                    },
                    "unset" => $k.to_string() + "=NULL,",
                    _ => $k.to_string() + "=" + value.as_str(),
                };
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str() + ")";

            sql
        }
    };

    ($t:expr, $i:expr, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let i_data = $i;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let tmp_s = $k.to_string() + "=" + value.as_str();
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str() + ")";

            sql
        }
    };
}
