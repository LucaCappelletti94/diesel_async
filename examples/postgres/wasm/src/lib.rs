//! Using `diesel-async` with the `wasm32-unknown-unknown` target.
//!
//! `AsyncPgConnection::establish` is unavailable on wasm because the target has
//! no TCP socket. The connection is instead built from a caller-provided
//! `tokio_postgres` client/connection pair (for example a WebSocket-backed
//! stream supplied by the host environment) via
//! [`AsyncPgConnection::try_from_client_and_connection`].

use diesel::sql_types::Integer;
use diesel::{ConnectionResult, IntoSql, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tokio::io::{AsyncRead, AsyncWrite};
use wasm_bindgen::prelude::*;

/// Builds an [`AsyncPgConnection`] from an already-established `tokio_postgres`
/// client and connection.
pub async fn connect<S, T>(
    client: tokio_postgres::Client,
    connection: tokio_postgres::Connection<S, T>,
) -> ConnectionResult<AsyncPgConnection>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    T: tokio_postgres::tls::TlsStream + Unpin + Send + 'static,
{
    AsyncPgConnection::try_from_client_and_connection(client, connection).await
}

/// Runs `SELECT 1` against the connection.
pub async fn select_one(conn: &mut AsyncPgConnection) -> QueryResult<i32> {
    diesel::select(1_i32.into_sql::<Integer>())
        .get_result(conn)
        .await
}

#[wasm_bindgen]
pub fn name() -> String {
    "diesel-async wasm example".to_string()
}
