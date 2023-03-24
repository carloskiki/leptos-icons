#[cfg(feature = "HiMdSolidUserCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidUserCircle")]
///This icon requires the feature `HiMdSolidUserCircle` to be enabled.
#[component]
pub fn UserCircle(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM12.5 7.5C12.5 8.88071 11.3807 10 10 10C8.61929 10 7.5 8.88071 7.5 7.5C7.5 6.11929 8.61929 5 10 5C11.3807 5 12.5 6.11929 12.5 7.5ZM10 12C8.04133 12 6.30187 12.9385 5.20679 14.3904C6.39509 15.687 8.1026 16.5 10 16.5C11.8974 16.5 13.6049 15.687 14.7932 14.3904C13.6981 12.9385 11.9587 12 10 12Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
