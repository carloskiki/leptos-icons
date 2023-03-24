#[cfg(feature = "TiArrowLeftThick")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiArrowLeftThick")]
/// *This icon requires the feature* `TiArrowLeftThick` *to be enabled*.
#[component]
pub fn ArrowLeftThick(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M18 11h-7.244l1.586-1.586c.781-.781.781-2.049 0-2.828-.781-.781-2.047-.781-2.828 0l-6.414 6.414 6.414 6.414c.39.391.902.586 1.414.586s1.023-.195 1.414-.586c.781-.781.781-2.049 0-2.828l-1.586-1.586h7.244c1.104 0 2-.896 2-2 0-1.105-.896-2-2-2z" /></svg>
   }
}