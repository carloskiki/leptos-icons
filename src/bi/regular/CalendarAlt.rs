#[cfg(feature = "BiRegularCalendarAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCalendarAlt")]
/// *This icon requires the feature* `BiRegularCalendarAlt` *to be enabled*.
#[component]
pub fn CalendarAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-2V2h-2v2H9V2H7v2H5a2 2 0 0 0-2 2zm16 14H5V8h14z" /></svg>
   }
}