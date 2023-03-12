#[cfg(feature = "FaSolidCheckToSlot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidCheckToSlot")]
/// *This icon requires the feature* `FaSolidCheckToSlot` *to be enabled*.
#[component]
pub fn CheckToSlot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M96 80c0-26.5 21.5-48 48-48H432c26.5 0 48 21.5 48 48V384H96V80zm313 47c-9.4-9.4-24.6-9.4-33.9 0l-111 111-47-47c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l64 64c9.4 9.4 24.6 9.4 33.9 0L409 161c9.4-9.4 9.4-24.6 0-33.9zM0 336c0-26.5 21.5-48 48-48H64V416H512V288h16c26.5 0 48 21.5 48 48v96c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V336z" /></svg>
   }
}