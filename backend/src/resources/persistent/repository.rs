use sea_orm::entity::prelude::*;
use sea_orm::Insert;

async fn create<T, DB>(data: T, db: &DB) -> Result<T, E>
    where
        DB: sea_orm::DatabaseConnection,
        T: Entity,
{
    let mut model = data.into_active_model();
    let id = db
        .insert(model)
        .await?
        .last_insert_id()
        .ok_or(E::DatabaseError)?;

    model.set_id(id);

    Ok(model.into())
}