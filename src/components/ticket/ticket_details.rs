use crate::models::TicketType;
use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn TicketDetails(ticket: TicketType) -> impl IntoView {
    let created = ticket.created_at_str();
    let updated = ticket.updated_at_str();
    let id = ticket.id;
    let status = ticket.status;
    let status_color = match status.as_str() {
        "In Progress" => "var(--yellow)",
        "Closed" => "var(--green)",
        _ => "var(--red)",
    };
    let priority = ticket.priority;
    let category = ticket.category;
    let description = ticket.description;

    view! {
      <div class="align-left">
        <div class="hstack space-between">
          <div class="hstack">
            <h4>"Ticket"</h4>
            #{id}
          </div>
          <div style=format!("color: {status_color}")>
            {status}
          </div>
        </div>

        <hr />
        <div class="hstack">
          <h5>"Priority"</h5>
          {priority}
        </div>
        <div class="hstack">
          <h5>"Category"</h5>
          {category}
        </div>
        <div class="hstack">
          <h5>"Creation Date"</h5>
          {created}
        </div>
        <div class="hstack">
          <h5>"Last Updated Date"</h5>
          {updated}
        </div>

        <hr />
        <h5>"Description"</h5>
        {description}
      </div>
    }
}
