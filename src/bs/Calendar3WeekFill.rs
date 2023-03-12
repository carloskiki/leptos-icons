#[cfg(feature = "BsCalendar3WeekFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsCalendar3WeekFill")]
/// *This icon requires the feature* `BsCalendar3WeekFill` *to be enabled*.
#[component]
pub fn Calendar3WeekFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-calendar3-week-fill" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M2 0a2 2 0 0 0-2 2h16a2 2 0 0 0-2-2H2zM0 14V3h16v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2zm12-8a1 1 0 1 0 2 0 1 1 0 0 0-2 0zM5 9a1 1 0 1 0 2 0 1 1 0 0 0-2 0zm5-2a1 1 0 1 1 0-2 1 1 0 0 1 0 2zM2 9a1 1 0 1 0 2 0 1 1 0 0 0-2 0z" /></svg>
   }
}