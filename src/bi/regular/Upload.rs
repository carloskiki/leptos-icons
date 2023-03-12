#[cfg(feature = "BiRegularUpload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularUpload")]
/// *This icon requires the feature* `BiRegularUpload` *to be enabled*.
#[component]
pub fn Upload(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 15h2V9h3l-4-5-4 5h3z" /><path d="M20 18H4v-7H2v7c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-7h-2v7z" /></svg>
   }
}