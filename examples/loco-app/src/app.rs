use async_trait::async_trait;
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
};
use migration::Migrator;
use sea_orm::{EntityTrait, PaginatorTrait};
use std::path::Path;

use crate::{
    controllers,
    models::_entities::{note_tags, notebooks, notes, tags, tasks, users},
    seed, views,
};

pub struct App;

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    // The one line loco-ui needs: the enhancement script and the capability beacon.
    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(loco_ui::loco::Initializer)])
    }

    /// Every path no route answers gets loco-ui's 404 page, in the app's look.
    async fn before_routes(_ctx: &AppContext) -> Result<axum::Router<AppContext>> {
        Ok(axum::Router::new().fallback(loco_ui::blocks::not_found))
    }

    /// Every page, the account pages and the 404 included, in the app's look.
    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        Ok(router.layer(views::look::LOOK.layer()))
    }

    /// The first start in development seeds the demo account (see `seed`), so a fresh
    /// checkout opens on notes rather than an empty page.
    async fn before_run(ctx: &AppContext) -> Result<()> {
        if ctx.environment == Environment::Development
            && users::Entity::find().count(&ctx.db).await? == 0
        {
            Self::seed(ctx, Path::new("src/fixtures")).await?;
            tracing::info!(
                "seeded the demo account: {} / {}",
                seed::EMAIL,
                seed::PASSWORD
            );
        }
        Ok(())
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::empty()
            .add_route(controllers::notebooks::routes())
            .add_route(controllers::tasks::routes())
            .add_route(controllers::account::routes())
            .add_route(controllers::notes::routes())
            .add_route(controllers::home::routes())
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(_tasks: &mut Tasks) {}

    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, note_tags::Entity).await?;
        truncate_table(&ctx.db, tags::Entity).await?;
        truncate_table(&ctx.db, notes::Entity).await?;
        truncate_table(&ctx.db, notebooks::Entity).await?;
        truncate_table(&ctx.db, tasks::Entity).await?;
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }

    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        seed::demo(&ctx.db).await
    }
}
