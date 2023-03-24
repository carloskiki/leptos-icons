#[cfg(feature = "IoPawSharp")]
use leptos::*;
#[cfg(feature = "IoPawSharp")]
///This icon requires the feature `IoPawSharp` to be enabled.
#[component]
pub fn PawSharp(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M442.8,361.82C434,336.72,413.49,324,393.69,311.7c-17.23-10.71-33.5-20.83-44.14-39C320.22,222.37,304.11,192,256.06,192s-64.21,30.38-93.61,80.69c-10.65,18.21-27,28.35-44.25,39.08-19.8,12.31-40.27,25-49.1,50.05A78.06,78.06,0,0,0,64,390.11C64,430.85,96.45,464,132.4,464s83.31-18.13,123.76-18.13S343.31,464,379.71,464,448,430.85,448,390.11A78.3,78.3,0,0,0,442.8,361.82Z"
        />< ellipse xmlns = "http://www.w3.org/2000/svg" cx = "72" cy = "216" rx = "56"
        ry = "72" />< ellipse xmlns = "http://www.w3.org/2000/svg" cx = "184" cy = "120"
        rx = "56" ry = "72" />< ellipse xmlns = "http://www.w3.org/2000/svg" cx = "328"
        cy = "120" rx = "56" ry = "72" />< ellipse xmlns = "http://www.w3.org/2000/svg"
        cx = "440" cy = "216" rx = "56" ry = "72" /> < title > { title } < / title > < /
        svg >
    }
}
