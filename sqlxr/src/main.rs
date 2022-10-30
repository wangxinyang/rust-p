use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{
    postgres::{types::PgRange, PgPoolOptions},
    types::Uuid,
};

#[derive(Debug)]
pub struct Course {
    pub id: Uuid,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDate>,
    pub timespan: PgRange<DateTime<Utc>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let connection_str = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://tosei@localhost:5432/tosei")
        .await?;
    println!("db_pool is : {:?}", pool);

    // 查询单个数据
    let course = sqlx::query_as!(Course, "select * from tr.course where name = $1", "mm")
        .fetch_one(&pool)
        .await?;
    println!("course is : {:?}", course);

    // 更新数据
    sqlx::query!(
        "update tr.course set name = $1 where teacher_id = $2",
        "mm",
        22
    )
    .execute(&pool)
    .await?;

    // 插入数据
    let start = Utc::now();
    let end = Utc::now() + chrono::Duration::days(1);
    let timespan: PgRange<DateTime<Utc>> = (start..end).into();
    sqlx::query!(
        "insert into tr.course (teacher_id, name, timespan) values ($1, $2, $3)",
        44,
        "tosei",
        timespan
    )
    .execute(&pool)
    .await?;

    // 查询所有
    let list = sqlx::query!("SELECT * FROM tr.course")
        .fetch_all(&pool)
        .await?;
    let mut vec = vec![];
    for row in list {
        let user = Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: row.time,
            timespan: row.timespan,
        };
        vec.push(user);
    }

    println!("数据库中的所有数据：{:#?}", vec);
    Ok(())
}
