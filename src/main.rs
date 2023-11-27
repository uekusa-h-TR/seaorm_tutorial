// main.rs

mod entities;
mod migrator;
use entities::{prelude::*, *};
use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::prelude::*;
use migrator::Migrator;
// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "postgres://postgres:password@localhost:5435";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend(){
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("{}/{}", DATABASE_URL, DB_NAME),
            ))
            .await?;
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(), 
                format!("DROP DATABASE IF EXISTS {}", DB_NAME)
                )).await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            )).await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    let schema_manager = SchemaManager::new(db); // To investigate the schema

    Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("chef").await?);

    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    // First find *La Boulangerie* as a Model
    let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
    .one(db)
    .await?
    .unwrap();

    let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).all(db).await?;
    let mut chef_names: Vec<String> = chefs.into_iter().map(|b| b.name).collect();
    chef_names.sort_unstable();

    assert_eq!(chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);

    // Inserting two bakeries and their chefs
    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    let la_id = bakery_res.last_insert_id;

    let arte_by_padaria = bakery::ActiveModel {
        name: ActiveValue::Set("Arte by Padaria".to_owned()),
        profit_margin: ActiveValue::Set(0.2),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(arte_by_padaria).exec(db).await?;
    for chef_name in ["Brian", "Charles", "Kate", "Samantha"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    let arte_id = bakery_res.last_insert_id;

    // would then need two sets of find_related to find 
    // First find bakeries as Models
    let bakeries: Vec<bakery::Model> = Bakery::find()
        .filter(
            Condition::any()
                .add(bakery::Column::Id.eq(la_id))
                .add(bakery::Column::Id.eq(arte_id))
        )
        .all(db)
        .await?;

    // Then use loader to load the chefs in one query.
    let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db).await?;
    let mut la_chef_names: Vec<String> = chefs[0].to_owned().into_iter().map(|b| b.name).collect();
    la_chef_names.sort_unstable();
    let mut arte_chef_names: Vec<String> = chefs[1].to_owned().into_iter().map(|b| b.name).collect();
    arte_chef_names.sort_unstable();

    assert_eq!(la_chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
    assert_eq!(arte_chef_names, ["Brian", "Charles", "Kate", "Samantha"]);
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}