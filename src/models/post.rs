use crate::schema::posts;

#[derive(Serialize)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct CreatePostReq {
    title: String,
    body: String
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct UpdatePostReq {
    id: i32,
    title: String,
    body: String,
    published: bool
}