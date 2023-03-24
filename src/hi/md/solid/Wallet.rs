#[cfg(feature = "HiMdSolidWallet")]
use leptos::*;
#[cfg(feature = "HiMdSolidWallet")]
///This icon requires the feature `HiMdSolidWallet` to be enabled.
#[component]
pub fn Wallet(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M1 4.24973C1.62675 3.77896 2.4058 3.5 3.25 3.5H16.75C17.5942 3.5 18.3733 3.77896 19 4.24973C18.9999 3.00721 17.9926 2 16.75 2H3.25C2.00745 2 1.00015 3.00721 1 4.24973Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1 7.24973C1.62675 6.77896 2.4058 6.5 3.25 6.5H16.75C17.5942 6.5 18.3733 6.77896 19 7.24973C18.9999 6.00721 17.9926 5 16.75 5H3.25C2.00745 5 1.00015 6.00721 1 7.24973Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 8C7.55228 8 8 8.44772 8 9C8 10.1046 8.89543 11 10 11C11.1046 11 12 10.1046 12 9C12 8.44772 12.4477 8 13 8H16.75C17.9926 8 19 9.00736 19 10.25V15.75C19 16.9926 17.9926 18 16.75 18H3.25C2.00736 18 1 16.9926 1 15.75V10.25C1 9.00736 2.00736 8 3.25 8H7Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
