use diesel::{self, result::QueryResult, query_dsl::RunQueryDsl, prelude::*};

use crate::schema::posts;
use crate::schema::posts::dsl::*;
use crate::DbConn;

#[derive(Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct CreatePostReq {
    title: String,
    body: String
}

pub struct UpdatePostReq {
    id: i32,
    title: String,
    body: String,
    published: bool
}

impl Post {
    pub async fn get_all(conn: &DbConn) -> QueryResult<Vec<Post>>{
        conn.run(|c| {
            posts.load::<Post>(c)
        }).await
    }

    pub async fn insert(p: CreatePostReq, conn: &DbConn) -> QueryResult<usize>{
        conn.run(move |c| {
            diesel::insert_into(posts::table).values(&p).execute(c)
        }).await
    }

    /*
    pub async fn get(postId: i32, conn: &DbConn) -> QueryResult<Post> {
        conn.run(|c| {
            posts.query().fitler(id.eq(postId)).first(c)
        }).await
    }
    */
}