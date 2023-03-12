#[cfg(feature = "RiOthersLineDoorClosed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiOthersLineDoorClosed")]
/// *This icon requires the feature* `RiOthersLineDoorClosed` *to be enabled*.
#[component]
pub fn DoorClosed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M3 21v-2h2V4c0-.552.448-1 1-1h12c.552 0 1 .448 1 1v15h2v2H3zM17 5H7v14h10V5zm-2 6v2h-2v-2h2z" /></g></svg>
   }
}