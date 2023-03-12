#[cfg(feature = "RiBusinessFillCalendarTodo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessFillCalendarTodo")]
/// *This icon requires the feature* `RiBusinessFillCalendarTodo` *to be enabled*.
#[component]
pub fn CalendarTodo(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17 3h4a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h4V1h2v2h6V1h2v2zM4 9v10h16V9H4zm2 2h2v2H6v-2zm0 4h2v2H6v-2zm4-4h8v2h-8v-2zm0 4h5v2h-5v-2z" /></g></svg>
   }
}