#[cfg(feature = "RiDeviceFillSave")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillSave")]
/// *This icon requires the feature* `RiDeviceFillSave` *to be enabled*.
#[component]
pub fn Save(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M18 21v-8H6v8H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h13l4 4v13a1 1 0 0 1-1 1h-2zm-2 0H8v-6h8v6z" /></g></svg>
   }
}