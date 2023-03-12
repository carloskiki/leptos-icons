#[cfg(feature = "RiDevelopmentFillBrackets")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDevelopmentFillBrackets")]
/// *This icon requires the feature* `RiDevelopmentFillBrackets` *to be enabled*.
#[component]
pub fn Brackets(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 3v2H6v14h3v2H4V3h5zm6 0h5v18h-5v-2h3V5h-3V3z" /></g></svg>
   }
}