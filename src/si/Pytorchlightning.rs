#[cfg(feature = "SiPytorchlightning")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPytorchlightning")]
/// *This icon requires the feature* `SiPytorchlightning` *to be enabled*.
#[component]
pub fn Pytorchlightning(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0L1.75 6v12L12 24l10.25-6V6zm-1.775 18l1.08-4.657-2.428-2.397L13.79 6l-1.082 4.665 2.414 2.384z" /></svg>
   }
}