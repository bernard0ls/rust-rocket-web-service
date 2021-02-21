use diesel::{self, prelude::*};

use rocket_contrib::json::Json;
use crate::schema::posts::dsl::*;

use crate::models::post::{Post, CreatePostReq};
use crate::{DbConn};
use crate::schema::posts;

#[get("/posts/get")]
pub fn get_posts(conn: DbConn) -> QueryResult<Json<Vec<Post>>> {
    posts
    .limit(5)
    .load::<Post>(&*conn)
    .map(|xs| Json(xs))
}

#[post("/posts/create", data="<newpost>")]
pub fn create_post(newpost: Json<CreatePostReq>, conn: DbConn) -> QueryResult<Json<Post>> {
    diesel::insert_into(posts::table)
    .values(&newpost.into_inner())
    .get_result(&*conn)
    .map(|x| Json(x))
}