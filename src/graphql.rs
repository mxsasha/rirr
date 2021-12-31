use async_graphql::*;

use crate::models::RpslObject;
use crate::sqldb::handler::{query_for_ip_more_specific, PgPool};

pub struct Query;

#[Object]
impl Query {
    async fn rpsl_objects(&self, ctx: &Context<'_>, prefix: String) -> Vec<RpslObject> {
        let conn = ctx.data::<PgPool>().unwrap().get().unwrap();
        query_for_ip_more_specific(&conn, &prefix)
    }
}
