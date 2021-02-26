use rocket_contrib::json::{JsonValue, Json};

use crate::models::post::{Post, CreatePostReq, UpdatePostReq};
use crate::{DbConn};

#[get("/posts/get")]
pub async fn get_posts(conn: DbConn) -> Result<JsonValue, JsonValue> {
   match Post::get_all(&conn).await {
        Ok(posts_vec) => Ok(json!({"posts": posts_vec})),
        Err(e) => {
            error_!("Failed to get posts from DB, error: {}", e);
            Err(json!({"error": "Something went wrong..."}))
        }
   }
}

#[get("/posts/get/<post_id>")]
pub async fn get_post(post_id: i32, conn: DbConn) -> Result<JsonValue, JsonValue> {
   match Post::get(post_id, &conn).await {
        Ok(post) => Ok(json!({"post": post})),
        Err(e) => {
            error_!("Failed to get post from DB, error: {}", e);
            Err(json!({"error": "Something went wrong..."}))
        }
   }
}

#[post("/posts/create", data="<newpost>")]
pub async fn create_post(newpost: Json<CreatePostReq>, conn: DbConn) -> Result<JsonValue, JsonValue> {
    let new_post = newpost.into_inner();
    match Post::insert(new_post, &conn).await {
        Ok(_) => Ok(json!({"message": "successfully created post"})),
        Err(e) => {
            error_!("Failed to insert post to DB, error: {}", e);
            Err(json!({"error": "Something went wrong..."}))
        }
    }
}

#[post("/posts/update", data="<updatepost>")]
pub async fn update_post(updatepost: Json<UpdatePostReq>, conn: DbConn) -> Result<JsonValue, JsonValue> {
    let update_post = updatepost.into_inner();
    match Post::update(update_post, &conn).await {
        Ok(_) => Ok(json!({"message": "successfully updated post"})),
        Err(e) => {
            error_!("Failed to update post to DB, error: {}", e);
            Err(json!({"error": "Something went wrong..."}))
        }
    }
}