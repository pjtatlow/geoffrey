// use crate::Error;
// use entity::{prelude::*, semesters};
// // use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
// use sea_orm::{ColumnTrait, EntityTrait};
// use sea_orm::{DatabaseConnection, QueryFilter};

// pub async fn get_active_semester(
//     db: &DatabaseConnection,
// ) -> Result<Option<semesters::Model>, Error> {
//     let s = Semesters::find()
//         .filter(semesters::Column::Active.eq(true))
//         .one(db)
//         .await?;

//     Ok(s)
// }
