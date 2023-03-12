#[cfg(feature = "SiFuturelearn")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFuturelearn")]
/// *This icon requires the feature* `SiFuturelearn` *to be enabled*.
#[component]
pub fn Futurelearn(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M22.081.61v7.566h-7.223v6.661H7.566v6.634H0v1.92h9.471v-6.649h7.306v-6.66H24V.61Z" /></svg>
   }
}