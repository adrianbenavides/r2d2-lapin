#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Lapin support for the r2d2 connection pool.

#[allow(missing_docs)]
pub mod prelude;

use futures_executor::block_on;
use lapin::protocol::{AMQPError, AMQPErrorKind, AMQPHardError};
use lapin::types::ShortString;
use lapin::{Connection, ConnectionProperties, ConnectionState, Error};

/// An `r2d2::ManageConnection` for `lapin::Connection`s.
///
/// # Example
/// ```no_run
/// use lapin::ConnectionProperties;
/// use r2d2_lapin::prelude::*;
/// use std::thread;
///
/// let manager = LapinConnectionManager::new("amqp://guest:guest@127.0.0.1:5672//", &ConnectionProperties::default());
/// let pool = r2d2::Pool::builder()
///     .max_size(15)
///     .build(manager)
///     .unwrap();
///
/// for _ in 0..20 {
///     let pool = pool.clone();
///     thread::spawn(move || {
///         let conn = pool.get().unwrap();
///         // use the connection
///         // it will be returned to the pool when it falls out of scope.
///     });
/// }
/// ```
#[derive(Debug)]
pub struct LapinConnectionManager {
    amqp_address: String,
    conn_properties: ConnectionProperties,
}

impl LapinConnectionManager {
    /// Initialise the connection manager with the data needed to create new connections.
    /// Refer to the documentation of [`lapin::ConnectionProperties`](https://docs.rs/lapin/1.2.8/lapin/struct.ConnectionProperties.html) for further details on the parameters.
    ///
    /// # Example
    /// ```
    /// let manager = r2d2_lapin::LapinConnectionManager::new("amqp://guest:guest@127.0.0.1:5672//", &lapin::ConnectionProperties::default());
    /// ```
    pub fn new(amqp_address: &str, conn_properties: &ConnectionProperties) -> Self {
        Self {
            amqp_address: amqp_address.to_string(),
            conn_properties: conn_properties.clone(),
        }
    }

    async fn async_connect(amqp_address: &str, conn_properties: ConnectionProperties) -> Result<Connection, Error> {
        lapin::Connection::connect(amqp_address, conn_properties).await
    }
}

impl r2d2::ManageConnection for LapinConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        block_on(Self::async_connect(&self.amqp_address, self.conn_properties.clone()))
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let valid_states = vec![ConnectionState::Initial, ConnectionState::Connecting, ConnectionState::Connected];
        if valid_states.contains(&conn.status().state()) {
            Ok(())
        } else {
            Err(Self::Error::ProtocolError(AMQPError::new(
                AMQPErrorKind::Hard(AMQPHardError::CONNECTIONFORCED),
                ShortString::from("Invalid connection"),
            )))
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        let broken_states = vec![ConnectionState::Closed, ConnectionState::Error];
        broken_states.contains(&conn.status().state())
    }
}
