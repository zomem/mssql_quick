/// 1.通过id，更新数据 ，返回 sql 语句。
/// ```
/// let sql = msupdate!("feedback", 50, {
///     "content": "这里有",
///     "uid": 77,
///     "des": "null",    // 表示更新该字段值为NULL
/// })  // id = 50
///
/// ms_run_vec(&mut client, sql).await.unwrap();
///
/// // 原子更新，(如果使用[字段，值]的方式，都所有都需要使用这种形式)
/// let sql2 = msupdate!("feedback", 50, {
///     "content": ["set", "更新"],  // set 就是替换操作
///     "uid": ["incr", -23],   // incr 原子性加减
///     "des": ["unset", ""]   // unset 清空值
/// })
///
/// ```
///
/// 2.通过指定字段的值，更新数据 ，返回 sql 语句。
/// ```
/// // uid = 12
/// let sql = msupdate!("feedback", {"uid": 12}, {"name": "zh"});
///
/// ms_run_vec(&mut client, sql).await.unwrap();
///
/// ```
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
                "&&str" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                _ => {
                    i_data.to_string() + ""
                }
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
                        "&&str" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        _ => {
                            temp_v.to_string() + ","
                        }
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

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str();

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
                "&&str" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                _ => {
                    i_data.to_string() + ""
                }
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
                        "&&str" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        _ => {
                            temp_v.to_string() + ","
                        }
                    };
                }
                let tmp_s = $k.to_string() + "=" + value.as_str();
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str();

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
                "&&str" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                _ => {
                    i_data.to_string() + ""
                }
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
                        "&&str" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        _ => {
                            temp_v.to_string() + ","
                        }
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

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str();

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
                "&&str" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string();
                    v_r = v_r.replace("'", "''");
                    "N'".to_string() + &v_r + "'"
                },
                _ => {
                    i_data.to_string() + ""
                }
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
                        "&&str" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string();
                            v_r = v_r.replace("'", "''");
                            "N'".to_string() + &v_r + "',"
                        },
                        _ => {
                            temp_v.to_string() + ","
                        }
                    };
                }
                let tmp_s = $k.to_string() + "=" + value.as_str();
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str();

            sql
        }
    };
}
