#[cfg(feature = "FaSolidSquareRootVariable")]
use leptos::*;
#[cfg(feature = "FaSolidSquareRootVariable")]
///This icon requires the feature `FaSolidSquareRootVariable` to be enabled.
#[component]
pub fn SquareRootVariable(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M282.6 78.1c8-27.3 33-46.1 61.4-46.1H544c17.7 0 32 14.3 32 32s-14.3 32-32 32H344L238.7 457c-3.6 12.3-14.1 21.2-26.8 22.8s-25.1-4.6-31.5-15.6L77.6 288H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H77.6c22.8 0 43.8 12.1 55.3 31.8l65.2 111.8L282.6 78.1zM393.4 233.4c12.5-12.5 32.8-12.5 45.3 0L480 274.7l41.4-41.4c12.5-12.5 32.8-12.5 45.3 0s12.5 32.8 0 45.3L525.3 320l41.4 41.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L480 365.3l-41.4 41.4c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L434.7 320l-41.4-41.4c-12.5-12.5-12.5-32.8 0-45.3z"
        /> < title > { title } < / title > < / svg >
    }
}
