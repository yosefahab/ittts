use crate::models::TicketType;
use leptos::prelude::*;

#[component]
pub fn TicketTable(
    tickets: ReadSignal<Vec<TicketType>>,
    on_ticket_click: WriteSignal<Option<TicketType>>,
    show_modal: SignalSetter<bool>,
) -> impl IntoView {
    let handle_click = move |ticket_id| {
        let selected_ticket = tickets.get().iter().find(|t| t.id == ticket_id).cloned();
        on_ticket_click.set(selected_ticket);
        show_modal.set(true);
    };

    view! {
      <table class="tickets-table">
        <thead class="tickets-table-header">
          <tr>
            <th class="tickets-table-header-cell">"ID"</th>
            <th class="tickets-table-header-cell">"Title"</th>
            <th class="tickets-table-header-cell">"Description"</th>
            <th class="tickets-table-header-cell">"Status"</th>
            <th class="tickets-table-header-cell">"Priority"</th>
            <th class="tickets-table-header-cell">"Category"</th>
            <th class="tickets-table-header-cell">"Created"</th>
            <th class="tickets-table-header-cell">"Last Updated"</th>
          </tr>
        </thead>
        <tbody>
          <For
            each=move || tickets.get()
            key=|ticket| ticket.id
            children=move |ticket| {
              view! {
                <tr class="tickets-table-row" on:click=move |_| handle_click(ticket.id)>
                  <td class="tickets-table-cell">{ticket.id.to_string()}</td>
                  <td class="tickets-table-cell">{ticket.title.clone()}</td>
                  <td class="tickets-table-cell">{ticket.description.clone()}</td>
                  <td class="tickets-table-cell">{ticket.status.clone()}</td>
                  <td class="tickets-table-cell">{ticket.priority.clone()}</td>
                  <td class="tickets-table-cell">{ticket.category.clone()}</td>
                  <td class="tickets-table-cell">{ticket.created_at_str()}</td>
                  <td class="tickets-table-cell">{ticket.updated_at_str()}</td>
                </tr>
              }
            }
          />
        </tbody>
      </table>
    }
}
