#[cfg(feature = "BsCone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsCone")]
/// *This icon requires the feature* `BsCone` *to be enabled*.
#[component]
pub fn Cone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-cone" viewBox="0 0 16 16"><path d="M7.03 1.88c.252-1.01 1.688-1.01 1.94 0l2.905 11.62H14a.5.5 0 0 1 0 1H2a.5.5 0 0 1 0-1h2.125L7.03 1.88z" /></svg>
   }
}