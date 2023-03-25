#[cfg(feature = "ImCoinPound")]
use leptos::*;
#[cfg(feature = "ImCoinPound")]
///This icon requires the feature `ImCoinPound` to be enabled.
#[component]
pub fn CoinPound(
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
        "M7.5 1c-4.142 0-7.5 3.358-7.5 7.5s3.358 7.5 7.5 7.5c4.142 0 7.5-3.358 7.5-7.5s-3.358-7.5-7.5-7.5zM7.5 14.5c-3.314 0-6-2.686-6-6s2.686-6 6-6c3.314 0 6 2.686 6 6s-2.686 6-6 6z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M9.5 11h-3.5v-2h1.5c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5h-1.5v-0.5c0-0.827 0.673-1.5 1.5-1.5 0.534 0 1.032 0.288 1.3 0.75 0.138 0.239 0.444 0.321 0.683 0.182s0.321-0.444 0.182-0.683c-0.446-0.771-1.276-1.25-2.165-1.25-1.378 0-2.5 1.122-2.5 2.5v0.5h-0.5c-0.276 0-0.5 0.224-0.5 0.5s0.224 0.5 0.5 0.5h0.5v3h4.5c0.276 0 0.5-0.224 0.5-0.5s-0.224-0.5-0.5-0.5z"
        /> < title > { title } < / title > < / svg >
    }
}
