use sea_orm::{Database, DatabaseConnection, DbErr, Schema, ConnectionTrait};
use crate::models::user::Entity as UserEntity;

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en el archivo .env");
    
    let db = Database::connect(&database_url).await?;
    
    // Auto-crear tablas si no existen
    setup_tables(&db).await?;
    
    Ok(db)
}

async fn setup_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    
    // Crear tabla de usuarios si no existe
    let stmt = builder.build(
        schema.create_table_from_entity(UserEntity).if_not_exists()
    );
    
    match db.execute(stmt).await {
        Ok(_) => tracing::info!("Tablas verificadas/creadas exitosamente"),
        Err(e) => tracing::warn!("Error al crear tablas: {}", e),
    }
    
    Ok(())
}