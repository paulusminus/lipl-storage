mod convert;
mod db;
mod handler;
mod sql;

pub use handler::{list, post, item, delete, put};

// pub fn router() -> Router {
//     Router::new()
//         .route("/", get(handler::list).post(handler::post))
//         .route("/:id", get(handler::item).delete(handler::delete).put(handler::put))
// }

