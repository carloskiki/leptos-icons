#[cfg(feature = "BiSolidAlbum")]
use leptos::*;
#[cfg(feature = "BiSolidAlbum")]
///This icon requires the feature `BiSolidAlbum` to be enabled.
#[component]
pub fn Album(
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
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm-4.607 8.055A4.956 4.956 0 0 0 7 12H5a6.978 6.978 0 0 1 2.051-4.95 6.978 6.978 0 0 1 2.225-1.5l.779 1.842c-.596.252-1.13.612-1.59 1.072s-.82.995-1.072 1.591zm4.6 3.945a2.007 2.007 0 1 1 0-4.014 2.007 2.007 0 0 1 0 4.014z"
        /> < title > { title } < / title > < / svg >
    }
}
