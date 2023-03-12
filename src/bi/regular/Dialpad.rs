#[cfg(feature = "BiRegularDialpad")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDialpad")]
/// *This icon requires the feature* `BiRegularDialpad` *to be enabled*.
#[component]
pub fn Dialpad(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 3h4v3h-4zm0 5h4v3h-4zm0 5h4v3h-4zm6-10h4v3h-4zm0 5h4v3h-4zm0 5h4v3h-4zM4 3h4v3H4zm0 5h4v3H4zm0 5h4v3H4zm6 5h4v3h-4z" /></svg>
   }
}