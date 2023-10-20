use tiberius::{error::Error, time::chrono::NaiveDateTime, Client, ColumnType, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};
pub use tiberius::{EncryptionLevel, Row};

// AsyncRead + AsyncWrite + Unpin + Send
pub struct MssqlQuick {
    pub client: Client<Compat<TcpStream>>,
}

impl MssqlQuick {
    pub async fn new(url: &str, encryp_level: EncryptionLevel) -> anyhow::Result<MssqlQuick> {
        let mut config = Config::from_ado_string(url)?;
        config.encryption(encryp_level);
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let client = match Client::connect(config, tcp.compat_write()).await {
            // Connection successful.
            Ok(client) => client,
            // The server wants us to redirect to a different address
            Err(Error::Routing { host, port }) => {
                let mut config = Config::from_ado_string(url)?;
                config.host(&host);
                config.port(port);
                config.encryption(encryp_level);

                let tcp = TcpStream::connect(config.get_addr()).await?;
                tcp.set_nodelay(true)?;

                // we should not have more than one redirect, so we'll short-circuit here.
                Client::connect(config, tcp.compat_write()).await?
            }
            Err(e) => Err(e)?,
        };

        Ok(MssqlQuick { client })
    }
}

/// 运行sql语句，返回想要的结果
/// ### 示例
/// ```
/// let sql = msget!("feedback", 33, "*");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Feedback {
///     id: u64,
///     cc: String
/// }
/// let res_get: Vec<Feedback> = ms_run_vec(&mut client, sql).await.unwrap();
/// ```
pub async fn ms_run_vec<T>(
    client: &mut Client<Compat<TcpStream>>,
    sql: String,
) -> anyhow::Result<Vec<T>>
where
    T: Serialize + DeserializeOwned,
{
    let res = client.simple_query(sql).await?.into_results().await?;

    if res.len() == 0 {
        return Ok(Vec::new());
    }

    let mut list_str = r#"["#.to_owned();
    for row in res[0].iter() {
        let columns = row.columns();
        let mut item = r#"{"#.to_owned();
        for index in 0..columns.len() {
            let f_type: ColumnType = columns[index].column_type();
            let f_name = columns[index].name();
            match f_type {
                ColumnType::Null => {
                    let val: Option<&str> = row.get(f_name);
                    match val {
                        Some(_v) => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Bit => {
                    let val: Option<bool> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Int1 => {
                    let val: Option<u8> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Int2 => {
                    let val: Option<i16> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Int4 => {
                    let val: Option<i32> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Int8 => {
                    let val: Option<i64> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Intn => {
                    let row_str = format!(r#"{:?}"#, row);
                    let re = Regex::new(r"TokenRow \{ data: \[(.*)\] \}, result_index: 0").unwrap();
                    let caps = re.captures(row_str.as_str()).unwrap();

                    let re_no = Regex::new(r"\(Some\(.*?\)\),").unwrap();
                    let no_value = re_no.replace_all(&caps[1], "");

                    let value: Vec<&str> = no_value.split(" ").collect();
                    let v_idx = value[index];
                    let mut val_str = "".to_owned();
                    if v_idx.contains("I64") {
                        let val: Option<i64> = row.get(f_name);
                        val_str = if let Some(v) = val {
                            format!("{}", v)
                        } else {
                            format!("null")
                        };
                    } else if v_idx.contains("I32") {
                        let val: Option<i32> = row.get(f_name);
                        val_str = if let Some(v) = val {
                            format!("{}", v)
                        } else {
                            format!("null")
                        };
                    } else if v_idx.contains("I16") {
                        let val: Option<i16> = row.get(f_name);
                        val_str = if let Some(v) = val {
                            format!("{}", v)
                        } else {
                            format!("null")
                        };
                    } else if v_idx.contains("U8") {
                        let val: Option<u8> = row.get(f_name);
                        val_str = if let Some(v) = val {
                            format!("{}", v)
                        } else {
                            format!("null")
                        };
                    }

                    item += format!(r#""{}":{},"#, f_name, val_str).as_str();
                }
                ColumnType::Float4 | ColumnType::Money4 => {
                    let val: Option<f32> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Float8 | ColumnType::Money => {
                    let val: Option<f64> = row.get(f_name);
                    match val {
                        Some(v) => {
                            item += format!(r#""{}":{},"#, f_name, v).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                ColumnType::Datetimen | ColumnType::Datetime2 | ColumnType::Datetime => {
                    let val: Option<NaiveDateTime> = row.get(index);
                    match val {
                        Some(v) => {
                            let date_str = v.to_string();
                            let v_c = serde_json::to_string(&date_str)?;
                            item += format!(r#""{}":{},"#, f_name, v_c).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
                _ => {
                    let val: Option<&str> = row.get(f_name);
                    match val {
                        Some(v) => {
                            let v_c = serde_json::to_string(&v)?;
                            item += format!(r#""{}":{},"#, f_name, v_c).as_str();
                        }
                        None => {
                            item += format!(r#""{}":null,"#, f_name).as_str();
                        }
                    }
                }
            }
        }
        item.pop();
        item += "},";
        list_str += item.as_str();
    }
    if res[0].len() > 0 {
        list_str.pop();
    }
    list_str += "]";
    let jsonvalue: Vec<T> = serde_json::from_str(list_str.as_str())?;
    Ok(jsonvalue)
}
