#[cfg(feature = "SiHyper")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiHyper")]
/// *This icon requires the feature* `SiHyper` *to be enabled*.
#[component]
pub fn Hyper(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M13.565 17.91H24v1.964H13.565zm-3.201-5.09l-9.187 8.003 2.86-7.004L0 11.179l9.187-8.002-3.11 7.451z" /></svg>
   }
}