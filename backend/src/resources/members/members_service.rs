pub mod members_service {
    use axum::http::StatusCode;
    use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
    use uuid::Uuid;

    use crate::models::user::NewUser;
    use crate::utils::errors::errors::APIError;

    pub async fn create_user<T>(db: DatabaseConnection, user_account: NewUser) -> Result<(), APIError> {
        println!("{:?} user data.", user_account);

        let user_model = entity::users::ActiveModel {
            name: Set(user_account.name.to_owned()),
            phone: Set(user_account.phone.to_owned()),
            password: Set(user_account.password.to_owned()),
            id: Set(Uuid::new_v4()),
            ..Default::default()
        };

        user_model.insert(&db).await
            .map_err(|err| APIError { message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50) })?;

        Ok(())
    }
}