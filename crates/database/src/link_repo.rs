use common::Link;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Error, PooledConnection};
use diesel::{OptionalExtension, SelectableHelper};

use crate::schema::links::short_code;
use crate::{
    PgPool,
    models::{DbLink, DbNewLink},
    schema::links,
};

pub struct LinkRepo {
    pool: PgPool,
}

impl LinkRepo {
    pub fn new(pool: PgPool) -> Self {
        LinkRepo { pool }
    }

    fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        self.pool.get().map_err(|_| {
            Error::ConnectionError(ConnectionError::BadConnection(
                "Failed to get connection".into(),
            ))
        })
    }

    pub fn create_link(&self, target_url: &str) -> Result<Link, Error> {
        let mut conn = self.conn()?;

        let res: DbLink = diesel::insert_into(links::table)
            .values(&DbNewLink { target_url })
            .returning(DbLink::as_returning())
            .get_result(&mut conn)
            .map_err(Error::QueryError)?;

        Ok(res.into())
    }

    pub fn get_link(&self, code: &str) -> Result<Option<Link>, Error> {
        let mut conn = self.conn()?;

        let result: Option<DbLink> = links::table
            .filter(short_code.eq(code))
            .select(DbLink::as_select())
            .first(&mut conn)
            .optional()
            .map_err(Error::QueryError)?;

        Ok(result.map(Into::into))
    }
}
