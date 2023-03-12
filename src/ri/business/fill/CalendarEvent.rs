#[cfg(feature = "RiBusinessFillCalendarEvent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessFillCalendarEvent")]
/// *This icon requires the feature* `RiBusinessFillCalendarEvent` *to be enabled*.
#[component]
pub fn CalendarEvent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17 3h4a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h4V1h2v2h6V1h2v2zM4 9v10h16V9H4zm2 4h5v4H6v-4z" /></g></svg>
   }
}