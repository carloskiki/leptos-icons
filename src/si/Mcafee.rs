#[cfg(feature = "SiMcafee")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMcafee")]
/// *This icon requires the feature* `SiMcafee` *to be enabled*.
#[component]
pub fn Mcafee(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 4.8233L1.5793 0v19.1767L12 24l10.4207-4.8233V0zm6.172 11.626l-6.143 2.8428-6.1438-2.8429V6.6894l6.1439 2.8418 6.1429-2.8418z" /></svg>
   }
}