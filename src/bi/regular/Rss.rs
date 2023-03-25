#[cfg(feature = "BiRegularRss")]
use leptos::*;
#[cfg(feature = "BiRegularRss")]
///This icon requires the feature `BiRegularRss` to be enabled.
#[component]
pub fn Rss(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19 20.001C19 11.729 12.271 5 4 5v2c7.168 0 13 5.832 13 13.001h2z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 20.001h2C14 14.486 9.514 10 4 10v2c4.411 0 8 3.589 8 8.001z" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "6" cy = "18" r = "2" /> < title > {
        title } < / title > < / svg >
    }
}
