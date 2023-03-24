#[cfg(feature = "VsSplitVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSplitVertical")]
/// *This icon requires the feature* `VsSplitVertical` *to be enabled*.
#[component]
pub fn SplitVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14 1H3L2 2v11l1 1h11l1-1V2l-1-1zm0 12H3V8h11v5zm0-6H3V2h11v5z" /></svg>
   }
}