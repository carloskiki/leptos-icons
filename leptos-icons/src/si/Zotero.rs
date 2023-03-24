#[cfg(feature = "SiZotero")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiZotero")]
/// *This icon requires the feature* `SiZotero` *to be enabled*.
#[component]
pub fn Zotero(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M21.231 2.462 7.18 20.923h14.564V24H2.256v-2.462L16.308 3.076H2.975V0h18.256v2.462z" /></svg>
   }
}