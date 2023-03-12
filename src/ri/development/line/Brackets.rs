#[cfg(feature = "RiDevelopmentLineBrackets")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDevelopmentLineBrackets")]
/// *This icon requires the feature* `RiDevelopmentLineBrackets` *to be enabled*.
#[component]
pub fn Brackets(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 3v2H6v14h3v2H4V3h5zm6 0h5v18h-5v-2h3V5h-3V3z" /></g></svg>
   }
}