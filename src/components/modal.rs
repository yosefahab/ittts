use leptos::prelude::*;

#[component]
pub fn Modal<F, C, V>(is_open: ReadSignal<bool>, children: F, on_close: C) -> impl IntoView
where
    F: Fn() -> V + 'static + Send + Sync,
    V: IntoView + 'static,
    C: Fn() + 'static + Send + Sync + Copy,
{
    view! {
      {move || {
        is_open.get().then(|| {
            view! {
              <div class="modal-overlay">
                <div class="modal-content">
                    <div class="modal-header">
                        <button class="modal-close-button" on:click=move |_| on_close()>"Close" </button>
                    </div>
                    <div class="modal-body">
                        {children()}
                    </div>
                </div>
              </div>
            }
          })
      }}
    }
}
