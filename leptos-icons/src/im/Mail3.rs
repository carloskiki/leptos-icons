#[cfg(feature = "ImMail3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMail3")]
/// *This icon requires the feature* `ImMail3` *to be enabled*.
#[component]
pub fn Mail3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.333 0h-10.666c-1.467 0-2.667 1.2-2.667 2.667v10.666c0 1.468 1.2 2.667 2.667 2.667h10.666c1.467 0 2.667-1.199 2.667-2.667v-10.666c0-1.467-1.2-2.667-2.667-2.667zM2.854 13.854l-1.207-1.207 4-4 0.457 0.457-3.25 4.75zM2.396 3.104l0.457-0.457 5.146 4.146 5.146-4.146 0.457 0.457-5.604 6.604-5.604-6.604zM13.146 13.854l-3.25-4.75 0.457-0.457 4 4-1.207 1.207z" /></svg>
   }
}