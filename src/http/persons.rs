use axum::{http::StatusCode, routing::get, Extension, Json, Router};

use super::{errors::internal_error, ApiContext};

pub fn router() -> Router {
    Router::new().route("/persons", get(persons))
}

#[derive(serde::Serialize)]
struct PersonSum {
    total_num_persons: usize,
    value: f64,
    avg_year: f64,
}

struct DbData {
    // Unused variable, the compiler will complain
    name: String,
    year: i64,
    value: f64,
}

fn map_db_to_res(db_data: Vec<DbData>) -> PersonSum {
    PersonSum {
        // Get length of vector
        total_num_persons: db_data.len(),
        // Converting from i64 to f64
        avg_year: db_data.iter().fold(0.0, |acc, x| acc + x.year as f64) / db_data.len() as f64,
        // Sum up values using functional approach
        value: db_data.iter().fold(0.0, |acc, x| acc + x.value),
    }
}

// Create endpoint for returning how many persons there are and the total value
async fn persons(
    ctx: Extension<ApiContext>,
) -> Result<(StatusCode, Json<PersonSum>), (StatusCode, String)> {
    let db_data = sqlx::query_as!(
        DbData,
        r#"
            SELECT name, year as "year!", value as "value!"
            FROM stoltit
            WHERE NOT(
                year IS NULL OR
                value IS NULL
            )
            "#,
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(internal_error)?;

    let mapped_data = map_db_to_res(db_data);

    Ok((StatusCode::OK, Json(mapped_data)))
}
