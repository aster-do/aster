use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sqlx::Row;

use crate::{
    dto::{
        billable::Billable,
        filters::{frequency::Frequency, operator::Operator, BillableFilter},
    },
    router::AppState,
};
// provides `try_next`
use futures::TryStreamExt;

fn field_error(e: sqlx::Error) -> (StatusCode, String) {
    log::error!("Database error: {}", e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Database error".to_string(),
    )
}

#[allow(unused_variables)]
pub async fn get_billables(
    State(state): State<Arc<AppState>>,
    Query(query): Query<BillableFilter>,
) -> Result<Json<Vec<Billable>>, (StatusCode, String)> {
    log::debug!("Received request for billables: {:?}", query);

    // refuse Total frequency if start or end is set

    if query.frequency == Frequency::Total && (query.start.is_some() || query.end.is_some()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Total frequency cannot have start or end set".to_string(),
        ));
    }

    let pool = &state.pool;

    let operator = match query.operator {
        Operator::Avg => "avg",
        Operator::Count => "count",
        Operator::Max => "max",
        Operator::Min => "min",
        Operator::Sum => "sum",
    };

    let frequency = match query.frequency {
        Frequency::Total => "total",
        Frequency::Hourly => "hour",
        Frequency::Daily => "day",
    };

    let mut sql_query = sqlx::QueryBuilder::new("SELECT name,");

    match query.frequency {
        Frequency::Total => {
            // the Total table has no timestamp column
            sql_query.push("CURRENT_TIMESTAMP as timestamp,");
        }
        Frequency::Hourly => {
            sql_query.push("timestamp,");
        }
        Frequency::Daily => {
            // the Daily table has a date instead of a timestamp type
            sql_query.push("timestamp::timestamp AT TIME ZONE 'UTC' as timestamp,");
        }
    }

    // get the right column for the operator

    sql_query.push(format!("{} as value ", operator));

    // query the right table

    match query.frequency {
        Frequency::Total => {
            sql_query.push("FROM billable_aggregate_oat ");
        }
        Frequency::Hourly => {
            sql_query.push("FROM billable_aggregate ");
        }
        Frequency::Daily => {
            sql_query.push("FROM billable_aggregate_by_day ");
        }
    }

    // add filters

    if query.start.is_some() || query.end.is_some() || query.name.is_some() {
        sql_query.push("WHERE ");

        let mut first = true;

        if let Some(name) = query.name {
            sql_query.push("name = ");
            sql_query.push_bind(name);
            first = false;
        }

        if let Some(start) = query.start {
            if !first {
                sql_query.push(" AND ");
            }
            sql_query.push("timestamp >= ");
            sql_query.push_bind(start);
            first = false;
        }

        if let Some(end) = query.end {
            if !first {
                sql_query.push(" AND ");
            }
            sql_query.push("timestamp <= ");
            sql_query.push_bind(end);
        }
    }

    // build and execute the query
    let built = sql_query.build();
    let mut rows = built.fetch(pool);

    let mut billables: Vec<Billable> = vec![];

    // iterate over the rows and build the billables

    while let Some(row) = rows.try_next().await.map_err(|e| {
        log::warn!("Error fetching billables: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error fetching billables".to_string(),
        )
    })? {
        // parsing has to be improved here

        let name: String = row.try_get(0).map_err(field_error)?;
        let timestamp: chrono::DateTime<Utc> = row.try_get(1).map_err(field_error)?;
        let value: f64 = row.try_get(2).map_err(field_error)?;

        let billable = Billable {
            name,
            timestamp,
            value,
        };

        billables.push(billable);
    }

    log::debug!("Returning billables: {:?}", billables);

    Ok(Json(billables))
}
