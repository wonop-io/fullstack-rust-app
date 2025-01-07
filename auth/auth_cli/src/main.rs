// Boilerplate code by Wonop ApS.

// This CLI application uses the clap crate for parsing command-line arguments
// and sqlx for database interactions with PostgreSQL.

use std::env;

use auth_api::User;
use clap::{Parser, Subcommand};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

// Define the main CLI structure using clap's derive macros
#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Troels Frimodt Rønnow <troels@wonop.com>",
    about = "User Management CLI"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

// Define the subcommands available in the CLI
#[derive(Subcommand)]
enum Command {
    List,
    Create {
        #[clap(required = true)]
        name: String,
        #[clap(required = true)]
        email: String,
        #[clap(default_value = "user")]
        role: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load environment variables from a .env file if present
    dotenvy::dotenv().ok();
    // Retrieve the DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Parse command-line arguments
    let cli = Cli::parse();

    // Match on the subcommand and call the appropriate function
    match cli.command {
        Command::List => {
            list_users(&pool).await?;
        }
        Command::Create { name, email, role } => {
            create_user(&pool, &name, &email, &role).await?;
        }
    }

    Ok(())
}

// Function to list all users from the database
async fn list_users(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Query the database for all users
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, username, role, created_at, updated_at
        FROM auth_users
        "#
    )
    .fetch_all(pool)
    .await?;

    // Print users in a formatted table
    println!(
        "{:<40} {:<30} {:<30} {:<10} {:<30} {:<30}",
        "ID", "Name", "Username", "Role", "Created At", "Updated At"
    );
    println!("{:-<170}", "");
    for user in users {
        println!(
            "{:<40} {:<30} {:<30} {:<10} {:<30} {:<30}",
            user.id.to_string(),
            user.name.unwrap_or_default(),
            user.username,
            user.role,
            user.created_at.map_or("N/A".to_string(), |dt| dt
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()),
            user.updated_at.map_or("N/A".to_string(), |dt| dt
                .format("%Y-%m-%d %H:%M:%S")
                .to_string())
        );
    }
    Ok(())
}

// Function to create a new user in the database
async fn create_user(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    role: &str,
) -> Result<(), sqlx::Error> {
    // Create a new User instance
    let new_user = User {
        id: Uuid::new_v4(),
        name: Some(name.to_string()),
        username: email.to_string(),
        role: role.to_string(),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    // Insert the new user into the database
    sqlx::query!(
        r#"
        INSERT INTO auth_users (id, name, username, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_user.id,
        new_user.name,
        new_user.username,
        new_user.role,
        new_user.created_at,
        new_user.updated_at
    )
    .execute(pool)
    .await?;

    println!("User created successfully: {:?}", new_user);
    Ok(())
}
