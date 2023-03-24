#[cfg(feature = "SiRiseup")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiRiseup")]
/// *This icon requires the feature* `SiRiseup` *to be enabled*.
#[component]
pub fn Riseup(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M10.5 24l-1.485-9.007-8.961-1.738L8.16 9.06 7.045 0l6.495 6.414 8.271-3.861-4.093 8.16 6.228 6.673-9.024-1.372z" /></svg>
   }
}