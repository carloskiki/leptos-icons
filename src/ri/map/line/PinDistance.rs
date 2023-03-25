#[cfg(feature = "RiMapLinePinDistance")]
use leptos::*;
#[cfg(feature = "RiMapLinePinDistance")]
///This icon requires the feature `RiMapLinePinDistance` to be enabled.
#[component]
pub fn PinDistance(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M9.975 8.975a3.5 3.5 0 1 0-4.95 0L7.5 11.45l2.475-2.475zM7.5 14.278L3.61 10.39a5.5 5.5 0 1 1 7.78 0L7.5 14.28zM7.5 8a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm9 12.45l2.475-2.475a3.5 3.5 0 1 0-4.95 0L16.5 20.45zm3.89-1.06l-3.89 3.888-3.89-3.889a5.5 5.5 0 1 1 7.78 0zM16.5 17a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
