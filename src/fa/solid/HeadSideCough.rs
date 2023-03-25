#[cfg(feature = "FaSolidHeadSideCough")]
use leptos::*;
#[cfg(feature = "FaSolidHeadSideCough")]
///This icon requires the feature `FaSolidHeadSideCough` to be enabled.
#[component]
pub fn HeadSideCough(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 224.2C0 100.6 100.2 0 224 0h32c95.2 0 174.2 69.3 189.4 160.1c2.2 13 6.7 25.7 15 36.1l42 52.6c6.2 7.8 9.6 17.4 9.6 27.4c0 24.2-19.6 43.8-43.8 43.8H448v0 32L339.2 365.6c-11 1.4-19.2 10.7-19.2 21.8c0 11.6 9 21.2 20.6 21.9L448 416v16c0 26.5-21.5 48-48 48H320v8c0 13.3-10.7 24-24 24H256v0H96c-17.7 0-32-14.3-32-32V407.3c0-16.7-6.9-32.5-17.1-45.8C16.6 322.4 0 274.1 0 224.2zm352-.2a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM464 384a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zm152-24a24 24 0 1 1 0 48 24 24 0 1 1 0-48zM592 480a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zM552 312a24 24 0 1 1 0 48 24 24 0 1 1 0-48zm40-24a24 24 0 1 1 48 0 24 24 0 1 1 -48 0zM552 408a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"
        /> < title > { title } < / title > < / svg >
    }
}
