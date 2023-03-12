#[cfg(feature = "TbCurrencyRupee")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCurrencyRupee")]
/// *This icon requires the feature* `TbCurrencyRupee` *to be enabled*.
#[component]
pub fn CurrencyRupee(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-currency-rupee" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 5h-11h3a4 4 0 0 1 0 8h-3l6 6" /><path d="M7 9l11 0" /></svg>
   }
}