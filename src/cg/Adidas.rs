#[cfg(feature = "CgAdidas")]
use leptos::*;
#[cfg(feature = "CgAdidas")]
///This icon requires the feature `CgAdidas` to be enabled.
#[component]
pub fn Adidas(
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
        "M1.3294 19L0.731323 17.9641L5.06145 15.4641L7.1029 19H1.3294Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.1858 19H9.4123L5.7935 12.7321L10.1236 10.2321L15.1858 19Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M23.2687 19H17.4952L10.8557 7.5L15.1858 5L23.2687 19Z" fill = "currentColor" />
        < title > { title } < / title > < / svg >
    }
}
