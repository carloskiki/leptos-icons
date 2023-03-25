#[cfg(feature = "RiLogosFillDouban")]
use leptos::*;
#[cfg(feature = "RiLogosFillDouban")]
///This icon requires the feature `RiLogosFillDouban` to be enabled.
#[component]
pub fn Douban(
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
        "M16.314 19.138h4.065a.62.62 0 0 1 .621.62v.621a.62.62 0 0 1-.62.621H3.62a.62.62 0 0 1-.62-.62v-.621a.62.62 0 0 1 .62-.621h3.754l-.96-3.104h2.19a.62.62 0 0 1 .59.425l.892 2.679H13.6l1.225-4.035H5.172a.62.62 0 0 1-.62-.62V7.345a.62.62 0 0 1 .62-.62h13.656a.62.62 0 0 1 .62.62v7.138a.62.62 0 0 1-.62.62h-1.289l-1.225 4.035zM3.931 3h16.138a.62.62 0 0 1 .62.62v.621a.62.62 0 0 1-.62.621H3.931a.62.62 0 0 1-.62-.62V3.62A.62.62 0 0 1 3.93 3zM7.19 8.586a.155.155 0 0 0-.156.155v4.035c0 .086.07.155.156.155h9.62c.086 0 .156-.07.156-.155V8.74a.155.155 0 0 0-.156-.155H7.19z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
