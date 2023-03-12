#[cfg(feature = "FaSolidChevronUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidChevronUp")]
/// *This icon requires the feature* `FaSolidChevronUp` *to be enabled*.
#[component]
pub fn ChevronUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M201.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L224 173.3 54.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z" /></svg>
   }
}