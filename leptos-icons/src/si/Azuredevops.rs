#[cfg(feature = "SiAzuredevops")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAzuredevops")]
/// *This icon requires the feature* `SiAzuredevops` *to be enabled*.
#[component]
pub fn Azuredevops(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 8.877L2.247 5.91l8.405-3.416V.022l7.37 5.393L2.966 8.338v8.225L0 15.707zm24-4.45v14.651l-5.753 4.9-9.303-3.057v3.056l-5.978-7.416 15.057 1.798V5.415z" /></svg>
   }
}