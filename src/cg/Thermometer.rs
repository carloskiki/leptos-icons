#[cfg(feature = "CgThermometer")]
use leptos::*;
#[cfg(feature = "CgThermometer")]
///This icon requires the feature `CgThermometer` to be enabled.
#[component]
pub fn Thermometer(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.9498 5.63615C17.3403 5.24563 17.9735 5.24563 18.364 5.63615C18.7545 6.02668 18.7545 6.65984 18.364 7.05037L11.2929 14.1214C10.9024 14.512 10.2693 14.512 9.87873 14.1214C9.48821 13.7309 9.48821 13.0977 9.87873 12.7072L16.9498 5.63615Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M7.82813 17.5862C9.7695 18.8725 12.4109 18.6603 14.1214 16.9499L21.1924 9.8788C23.1451 7.92617 23.1451 4.76035 21.1924 2.80773C19.2398 0.855106 16.074 0.855106 14.1214 2.80773L7.0503 9.8788C5.33984 11.5893 5.12771 14.2307 6.41392 16.172L2.80766 19.7783C2.41714 20.1688 2.41714 20.802 2.80766 21.1925C3.19819 21.583 3.83135 21.583 4.22188 21.1925L7.82813 17.5862ZM12.7072 15.5356L19.7782 8.46458C20.9498 7.29301 20.9498 5.39351 19.7782 4.22194C18.6067 3.05037 16.7072 3.05037 15.5356 4.22194L8.46452 11.293C7.29294 12.4646 7.29294 14.3641 8.46452 15.5356C9.63609 16.7072 11.5356 16.7072 12.7072 15.5356Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
