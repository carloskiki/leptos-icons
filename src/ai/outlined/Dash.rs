#[cfg(feature = "AiOutlinedDash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedDash")]
/// *This icon requires the feature* `AiOutlinedDash` *to be enabled*.
#[component]
pub fn Dash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M112 476h160v72H112zm320 0h160v72H432zm320 0h160v72H752z" /></svg>
   }
}