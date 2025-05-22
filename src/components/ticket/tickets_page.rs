use crate::components::ticket::{CreateTicket, TicketDetails, TicketTable};
use crate::components::Modal;
use crate::models::TicketType;
use leptos::prelude::*;

#[component]
pub fn TicketsPage() -> impl IntoView {
    let (tickets, set_tickets) = signal(vec![
        TicketType::new(1),
        TicketType {
            id: 2,
            title: "Fingerprint registration".to_string(),
            description: "Need to register finger for attendance".to_string(),
            status: "In Progress".to_string(),
            priority: "Medium".to_string(),
            category: "Other".to_string(),
            ..Default::default()
        },
        TicketType {
            id: 3,
            title: "Faulty laptop".to_string(),
            description:
                "Some very very very very very long and boring description about a faulty laptop"
                    .to_string(),
            status: "In Progress".to_string(),
            priority: "High".to_string(),
            category: "Hardware".to_string(),
            ..Default::default()
        },
    ]);
    let (is_create_modal_shown, set_create_modal_shown) = signal(false);
    let (is_ticket_modal_shown, set_ticket_modal_shown) = signal(false);
    let (selected_ticket, set_selected_ticket) = signal(None);
    let add_ticket = move || {
        set_tickets.update(|list| list.push(TicketType::new(list.len() + 1)));
        set_create_modal_shown(false)
    };

    view! {
      <div>
        <div class="hstack align-right">
          <button on:click=move |_| log::info!("Refresh")>"Refresh"</button>
          <button on:click=move |_| {set_create_modal_shown(true);}>"Create Ticket"</button>
        </div>

        <TicketTable
          tickets=tickets
          on_ticket_click=set_selected_ticket
          show_modal=set_ticket_modal_shown.into()
        />
      </div>

      <Modal
        is_open=is_ticket_modal_shown
        children=move || {
          selected_ticket.get().map(|t| {
              view! { <TicketDetails ticket=t /> }
            })
        }
        on_close=move || set_ticket_modal_shown(false)
      />

      <Modal
        is_open=is_create_modal_shown
        children=move || view! { <CreateTicket on_submit=add_ticket /> }
        on_close=move || set_create_modal_shown(false)
      />
    }
}
