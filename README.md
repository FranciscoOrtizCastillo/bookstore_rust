# Creación de una API en Rust

Del articulo :
https://developer.oracle.com/es/learn/technical-articles/1482042218497-creación-de-api-en-rust-on-oci

```bash
mkdir bookstore_rust
cd bookstore_rust
code .

cargo run
# open http://127.0.0.1:3000


curl http://localhost:3000/books
```

## Ejemplo de server.rs inicial :

```
#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
```

## Dockerizing the Rust server

```bash
docker build -t server_rust . 

docker run -p 3000:3000 --rm --name server_docker server_rust

```