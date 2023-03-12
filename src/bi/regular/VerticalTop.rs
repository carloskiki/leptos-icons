#[cfg(feature = "BiRegularVerticalTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularVerticalTop")]
/// *This icon requires the feature* `BiRegularVerticalTop` *to be enabled*.
#[component]
pub fn VerticalTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12 7-4 4h3v8h2v-8h3l-4-4zM3 3h3v2H3zM8 3h3v2H8zM13 3h3v2h-3zM18 3h3v2h-3z" /></svg>
   }
}