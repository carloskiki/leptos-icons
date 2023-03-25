#[cfg(feature = "SiPiwigo")]
use leptos::*;
#[cfg(feature = "SiPiwigo")]
///This icon requires the feature `SiPiwigo` to be enabled.
#[component]
pub fn Piwigo(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.712 12.777A4.713 4.713 0 0 1 12 17.49a4.713 4.713 0 0 1-4.713-4.713A4.713 4.713 0 0 1 12 8.066a4.713 4.713 0 0 1 4.712 4.711zm2.4-12.776c-2.572.058-2.358 1.544-8.237 1.555h-4.15A5.947 5.947 0 0 0 .777 7.503v10.55A5.947 5.947 0 0 0 6.725 24h10.55a5.947 5.947 0 0 0 5.948-5.947V4.081l-.008-.018c0-.014.004-.028.004-.043 0-2.227-1.88-4.02-4.108-4.02zm.09 2.545a1.409 1.409 0 0 1 1.407 1.41A1.409 1.409 0 0 1 19.2 5.364a1.409 1.409 0 0 1-1.41-1.408 1.409 1.409 0 0 1 1.41-1.41zM12 6.12a6.656 6.656 0 0 1 6.656 6.655A6.656 6.656 0 0 1 12 19.434a6.656 6.656 0 0 1-6.656-6.657A6.656 6.656 0 0 1 12 6.122z"
        /> < title > { title } < / title > < / svg >
    }
}
