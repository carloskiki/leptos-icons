#[cfg(feature = "ImMeter2")]
use leptos::*;
#[cfg(feature = "ImMeter2")]
///This icon requires the feature `ImMeter2` to be enabled.
#[component]
pub fn Meter2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM4.732 13.034c0.174-0.479 0.268-0.995 0.268-1.534 0-1.943-1.231-3.598-2.956-4.228 0.16-1.327 0.754-2.555 1.714-3.514 1.133-1.133 2.64-1.757 4.243-1.757s3.109 0.624 4.243 1.757c0.96 0.96 1.554 2.188 1.714 3.514-1.725 0.63-2.956 2.285-2.956 4.228 0 0.539 0.095 1.055 0.268 1.534-0.964 0.629-2.090 0.966-3.268 0.966s-2.304-0.338-3.268-0.966zM8.621 10.016c0.217 0.055 0.379 0.251 0.379 0.484v1c0 0.275-0.225 0.5-0.5 0.5h-1c-0.275 0-0.5-0.225-0.5-0.5v-1c0-0.233 0.162-0.43 0.379-0.484l0.371-7.016h0.5l0.371 7.016z"
        /> < title > { title } < / title > < / svg >
    }
}
