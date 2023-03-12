#[cfg(feature = "BiRegularHorizontalLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularHorizontalLeft")]
/// *This icon requires the feature* `BiRegularHorizontalLeft` *to be enabled*.
#[component]
pub fn HorizontalLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m11 8-4 4 4 4v-3h8v-2h-8V8zM3 18h2v3H3zM3 13h2v3H3zM3 8h2v3H3zM3 3h2v3H3z" /></svg>
   }
}