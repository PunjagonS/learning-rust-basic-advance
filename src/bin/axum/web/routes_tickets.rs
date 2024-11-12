use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

use crate::error::{Error, Result};
use crate::model::{ModelController, Ticket, TicketForCreate};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc) // Make sure all of these handlers can get the states
}

// region:    --- REST Handlers
async fn create_ticket(
    /*
        Use application state to be created and shared
        across all the handlers.
    */
    State(mc): State<ModelController>, // Extractor for application state
    Json(ticket_fc): Json<TicketForCreate>, // Extractor for ticket data from request body
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}

// endregion: --- REST Handlers
