#[cfg(feature = "SiZerodha")]
use leptos::*;
#[cfg(feature = "SiZerodha")]
///This icon requires the feature `SiZerodha` to be enabled.
#[component]
pub fn Zerodha(
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
        "M20.378 5.835A27.267 27.267 0 0124 12.169V0H13.666c2.486 1.343 4.763 3.308 6.712 5.835zM5.48 1.297c-1.914 0-3.755.409-5.48 1.166V24h22.944C22.766 11.44 15 1.297 5.48 1.297z"
        /> < title > { title } < / title > < / svg >
    }
}
