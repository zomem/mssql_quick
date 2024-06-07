### mssql 数据库连接方法封装

```rust
use mssql_quick::{
    ms_run_vec, mscount, msdel, msfind, msget, msset, mssetmany, msupdate, msupdatemany,
    EncryptionLevel, MssqlQuick, MssqlQuickSet,
};

const MSSQL_URL: &str = "server=tcp:localhost,1433;user=SA;password=ji83laa;database=dev_db;IntegratedSecurity=true;TrustServerCertificate=true";

let mut client = MssqlQuick::new(MSSQL_URL, EncryptionLevel::NotSupported)
    .await
    .unwrap()
    .client;
```

### mssql sql执行

| 运行sql    | 说明                                           |
| ---------- | ---------------------------------------------- |
| ms_run_vec | 执行sql，返回vec类型数据，无数据则返回`vec![]` |

```rust
// 执行 sql 语句
let data: Vec<serde_json::Value> = ms_run_vec(&mut conn, sql).unwrap();
```

### sql快捷生成

| sql快捷生成方法 | 说明                    |
| --------------- | ----------------------- |
| mscount         | 返回计数的sql           |
| msdel           | 删除一条数据的sql       |
| msdelmany       | 批量删除数据的sql       |
| msfind          | 查寻数据的sql           |
| msget           | 查寻一条数据的sql       |
| msset           | 新增一条数据的sql       |
| mssetmany       | 批量新增数据的sql       |
| msupdate        | 更新一条数据的sql       |
| msupdatemany    | 批量更新数据的sql       |
| 自定义          | 可以直接写自己的sql语句 |

以下内容，则为常用sql的快捷方法

```rust

// 新增一条数据
ms_run_vec(&mut client, msset!("for_test", {
    "content": "ADFaadf",
    "uid": 9,
    "info": if let Some(a) = one_info {a} else {"null"},
})).await.unwrap();

// 删除一条数据
ms_run_vec(&mut client, msdel!("for_test", 50)).await.unwrap();

// 更新一条数据
ms_run_vec(&mut client, msupdate!("for_test", 56, {
    "content": "更新后的内容，一一一一"
})).await.unwrap();

// 批量 新增数据
mssetmany!("for_test", vec![
    Item {uid: 1, content: "批量更新00adf"},
    Item {uid: 2, content: "2342341"},
    Item {uid: 3, content: "mmmmm"},
])
ms_run_vec(&mut client, msql).await.unwrap();

// 批量 更新数据
let sql = msupdatemany!("for_test", "uid", vec![
    Item {uid: 1, content: "批量更新00adf"},
    Item {uid: 2, content: "2342341"},
])
ms_run_vec(&mut client, sql).await.unwrap();



// 获取一条数据
let sql1 = msget!("for_test", 33, "id, content as cc");
#[derive(Serialize, Deserialize, Debug)]
struct Feedback {
    id: u64,
    cc: String
}
let res_get: Vec<Feedback> = ms_run_vec(&mut client, sql1).await.unwrap();

// 查寻数据
let sql_f = msfind!("for_test", {
    p0: ["uid", ">", 330],
    r: "p0",
    select: "id,content as cc",
});
let res_find: Vec<Feedback> = ms_run_vec(&mut client, sql_f).await.unwrap();

// 获取计数
let res_count: Vec<MssqlQuickCount> = ms_run_vec(&mut client, mscount!("for_test", {})).await.unwrap();

// 自定义查寻
let list: Vec<serde_json::Value> =
    ms_run_vec(&mut client, "select distinct type_v3 from dishes".to_owned()).await.unwrap();

```

### 组合查寻
通过 Sql 包裹
```rust
use mssql_quick::Sql;

let sql1 = msfind!("Hospital", {
    p0: ["HospitalName", "like", "信息%"],
    r: "p0",
    select: "HospitalId",
});
let sql2 = mscount!("DataBase..Patient", { // 对其他库的表查寻
    p0: ["InvestigationId", "=", Sql("Investigation.InvestigationId")],
    r: "p0",
});

let sql = msfind!("Investigation", {
    j1: ["HospitalId", "inner", "Hospital.HospitalId"],
    p0: ["HospitalId", "in", Sql(sql1)],
    p1: ["InvType", "=", "门诊"],
    r: "p0 && p1",
    select: "InvestigationId, HospitalId, (".to_string()
        + sql2.as_str() + ") as patient_count",  // 如果自己写sql语句，要注意sql注入
});

println!("sql>>>>>  {} \n", sql);
```
