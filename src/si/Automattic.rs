#[cfg(feature = "SiAutomattic")]
use leptos::*;
#[cfg(feature = "SiAutomattic")]
///This icon requires the feature `SiAutomattic` to be enabled.
#[component]
pub fn Automattic(
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
        "M14.521 8.11a1.497 1.497 0 01.433 2.102l-3.511 5.441a1.496 1.496 0 01-2.068.457 1.507 1.507 0 01-.44-2.08l3.513-5.44c.215-.335.554-.57.943-.655.39-.085.796-.04 1.13.175z M11.98 23.03C4.713 23.03 0 17.79 0 12.338v-.676C0 6.117 4.713.97 11.98.97 19.246.97 24 6.117 24 11.662v.676c0 5.453-4.713 10.692-12.02 10.692zm8.133-11.31c0-3.974-2.888-7.51-8.133-7.51-5.245 0-8.087 3.542-8.087 7.51v.497c0 3.974 2.888 7.578 8.087 7.578 5.198 0 8.133-3.604 8.133-7.578v-.497z"
        /> < title > { title } < / title > < / svg >
    }
}
