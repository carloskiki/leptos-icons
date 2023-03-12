#[cfg(feature = "RiBusinessFillBarChart2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessFillBarChart2")]
/// *This icon requires the feature* `RiBusinessFillBarChart2` *to be enabled*.
#[component]
pub fn BarChart2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 13h6v8H2v-8zM9 3h6v18H9V3zm7 5h6v13h-6V8z" /></g></svg>
   }
}