#[cfg(feature = "SiAccenture")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAccenture")]
/// *This icon requires the feature* `SiAccenture` *to be enabled*.
#[component]
pub fn Accenture(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M23.297 14.74L.434 24v-5.263L16.8 12.11l6.497 2.631zm.27-5.371L.433 0v5.263l23.132 9.368V9.37z" /></svg>
   }
}