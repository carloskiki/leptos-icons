#[cfg(feature = "SiTruenas")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiTruenas")]
/// *This icon requires the feature* `SiTruenas` *to be enabled*.
#[component]
pub fn Truenas(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 10.049v5.114l-10.949 6.324v-5.114L24 10.049zm-24 0v5.114l10.956 6.324v-5.114L0 10.049zm12.004-.605l-4.433 2.559 4.433 2.559 4.429-2.559-4.429-2.559zm10.952-1.207l-9.905-5.723v5.118l5.473 3.164 4.432-2.559zm-12-.605V2.513L1.044 8.236l4.432 2.555 5.48-3.159z" /></svg>
   }
}