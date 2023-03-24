#[cfg(feature = "CgCornerDoubleDownLeft")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleDownLeft")]
///This icon requires the feature `CgCornerDoubleDownLeft` to be enabled.
#[component]
pub fn CornerDoubleDownLeft(
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
        "M11.2952 7.69432L16.1357 2.6377L21.1924 7.47821L19.8094 8.92296L17.3474 6.56617L17.5094 12.6C17.5805 15.25 15.49 17.456 12.8399 17.5271L6.91369 17.6863L9.20765 19.9335L7.80805 21.3622L2.80768 16.4636L7.70628 11.4632L9.13495 12.8628L6.75759 15.2896L12.7755 15.128C14.1005 15.0924 15.1458 13.9895 15.1102 12.6645L14.9519 6.76668L12.74 9.07732L11.2952 7.69432Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
