use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use rand::Rng;

// creazione dummy data per 
#[derive(Serialize)]
struct Point {
    name: String,
    value: i32,
}

#[get("/api/data")]
async fn get_dummy_data() -> impl Responder {
    let mut rng = rand::rng();

    // numero casuale tra 3 e 12
    let num_graphs = rng.random_range(3..=12);

    let dummy_data: Vec<Vec<Point>> = (0..num_graphs)
        .map(|_| {
            vec![
                Point {
                    name: "A".into(),
                    value: rng.random_range(1..=50),
                },
                Point {
                    name: "B".into(),
                    value: rng.random_range(1..=50),
                },
                Point {
                    name: "C".into(),
                    value: rng.random_range(1..=50),
                },
            ]
        })
        .collect();

    HttpResponse::Ok().json(dummy_data)
}