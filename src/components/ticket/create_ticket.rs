use leptos::{html::*, prelude::*};
use leptos_router::components::Form;

#[allow(non_snake_case)]
#[component]
pub fn CreateTicket(on_submit: impl Fn() + 'static + Copy) -> impl IntoView {
    let title = NodeRef::<Input>::new();
    let status = NodeRef::<Input>::new();
    let priority = NodeRef::<Input>::new();
    let category = NodeRef::<Input>::new();
    let description = NodeRef::<Textarea>::new();

    view! {
      <h3>Create Ticket</h3>
      <Form method="post" action="/create-ticket">
        <div class="form-group">
          // <label>"Title:"</label>
          <input type="text" node_ref=title name="title" required=true placeholder="title"/>
        </div>
        <div class="form-group">
          // <label>"Status:"</label>
          <input type="text" node_ref=status name="status" placeholder="text"/>
        </div>
        <div class="form-group">
          // <label>"Priority:"</label>
          <input type="" node_ref=priority name="priority" placeholder="priority"/>
        </div>
        <div class="form-group">
          // <label>"Category:"</label>
          <input type="text" node_ref=category name="category" placeholder="category"/>
        </div>
        <div class="form-group">
          // <label>"Description:"</label>
          <textarea rows=1 node_ref=description name="description" required=true placeholder="description"></textarea>
        </div>
        <button type="submit">"Create Ticket"</button>
      </Form>
    }
}
