use axum:: {
    routing::{get},
    extract::Path,
    http::StatusCode,
    response::IntoResponse,Json,
    Router
};



use std::net::SocketAddr;
use serde:: {Deserialize, Serialize};

use serde_json::json;

use tower_http::{trace::TraceLayer};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Default,Serialize,Deserialize, Debug)]
struct JSONResponse {
     total_count: i64,
     incomplete_results: bool,
     items : Vec<Items>,
}

#[derive(Default,Serialize,Deserialize, Debug, Hash, Eq, PartialEq)]
struct Items {
     url: String,
     repository_url: String,
}

#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    //tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/issues/:tag", get(get_issue))
        .route("/issues/", get(get_issues))
        .route("/search/", get(search_issues))
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn get_issues() -> impl IntoResponse {
    
    let request_url ="https://api.github.com/search/issues?q=language:rust";
    

    let response = minreq::get(request_url)
     .with_header("user-agent", "github client")
     .send();

     let issues: JSONResponse = response.unwrap().json().unwrap();

    
    Json(issues)
}

async fn get_issue(Path(tag): Path<String>) -> impl IntoResponse {
    let issue = tag.as_str();
    let this_issue = String::from("You want issues with this tag:");
    
    (StatusCode::OK, Json(json!({"message":this_issue+issue})))
}

async fn search_issues() -> impl IntoResponse {
     let request_url ="https://api.github.com/search/issues?q=language:rust";
    

     match minreq::get(request_url).with_header("user-agent", "github client").send() {
            Ok(response) => {
                if response.status_code == 200 {
                    Json(json!({"message":"OK"}));

                } else if response.status_code == 404 {
                    Json(json!({"message":"Not Found"}));

                }
            },        
            Err(_err) => {
                Json(json!({"message":"Sorry, problems with the server"}));
        }
     }
     
    
}