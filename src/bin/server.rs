use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Book {
    title: String,
    author: String,
    year: u32,
}

use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<Vec<Book>>>;

#[tokio::main]
async fn main() {
    let mut book_catalog: Vec<Book> = Vec::new();
    book_catalog.push(Book {
        title: "The Hitchhiker's Guide to the Galaxy".to_string(),
        author: "Douglas Adams".to_string(),
        year: 1979,
    });
    book_catalog.push(Book {
        title: "The Restaurant at the End of the Universe".to_string(),
        author: "Douglas Adams".to_string(),
        year: 1980,
    });
    book_catalog.push(Book {
        title: "Life, the Universe and Everything".to_string(),
        author: "Douglas Adams".to_string(),
        year: 1982,
    });
    book_catalog.push(Book {
        title: "So Long, and Thanks for All the Fish".to_string(),
        author: "Douglas Adams".to_string(),
        year: 1984,
    });
    book_catalog.push(Book {
        title: "Mostly Harmless".to_string(),
        author: "Douglas Adams".to_string(),
        year: 1992,
    });

    let db = Arc::new(Mutex::new(book_catalog));

    warp::serve(filters::construct_book_routes(db))
        .run(([0, 0, 0, 0], 3000))
        .await;
}

mod filters {
    use super::Db;
    use super::Book;
    use super::handlers;
    use warp::Filter;
    use std::convert::Infallible;

    /// The routes, combined.
    pub fn construct_book_routes(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        route_get_books(db.clone())
            .or(route_post_books(db.clone()))
            .or(route_delete_book(db.clone()))
    }

    /// GET /books
    pub fn route_get_books(
        db:  Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("books")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::get_books)
    }

    /// POST /books with JSON body
    pub fn route_post_books(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("books")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_book)
    }

    /// DELETE /books/:id
    pub fn route_delete_book(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("books" / u64)
            .and(warp::delete())
            .and(with_db(db))
            .and_then(handlers::delete_book)
    }

    pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    pub fn json_body() -> impl Filter<Extract = (Book,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::Db;
    use super::Book;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_books(db: Db) -> Result<impl warp::Reply, Infallible> {
        let books = db.lock().await;
        let books: Vec<Book> = books.clone();
        Ok(warp::reply::json(&books))
    }

    pub async fn create_book(
        book: Book,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        let mut books = db.lock().await;

        books.push(book);

        Ok(StatusCode::CREATED)
    }

    pub async fn delete_book(
        id: u64,
        db: Db) -> Result<impl warp::Reply, Infallible> {
        let mut books = db.lock().await;

        let mut iter = 0;
        let len = books.len();
        books.retain(|_book| {
            let mut keep = true;
            if iter == id {
                iter += 1;
                keep = false;
            }
            iter += 1;
            keep
        });

        // If the vec is smaller, we found and deleted a book!
        let deleted = books.len() != len;

        if deleted {
            // respond with a `204 No Content`, which means successful,
            Ok(StatusCode::NO_CONTENT)
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
    }
}