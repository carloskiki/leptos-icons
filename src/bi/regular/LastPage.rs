#[cfg(feature = "BiRegularLastPage")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLastPage")]
/// *This icon requires the feature* `BiRegularLastPage` *to be enabled*.
#[component]
pub fn LastPage(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7.707 17.707 13.414 12 7.707 6.293 6.293 7.707 10.586 12l-4.293 4.293zM15 6h2v12h-2z" /></svg>
   }
}