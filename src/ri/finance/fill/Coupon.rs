#[cfg(feature = "RiFinanceFillCoupon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceFillCoupon")]
/// *This icon requires the feature* `RiFinanceFillCoupon` *to be enabled*.
#[component]
pub fn Coupon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 9.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v5.5a2.5 2.5 0 1 0 0 5V20a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1v-5.5a2.5 2.5 0 1 0 0-5zM9 9v2h6V9H9zm0 4v2h6v-2H9z" /></g></svg>
   }
}