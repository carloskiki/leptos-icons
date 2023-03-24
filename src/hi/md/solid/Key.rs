#[cfg(feature = "HiMdSolidKey")]
use leptos::*;
#[cfg(feature = "HiMdSolidKey")]
///This icon requires the feature `HiMdSolidKey` to be enabled.
#[component]
pub fn Key(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M8 7C8 4.23858 10.2386 2 13 2C15.7614 2 18 4.23858 18 7C18 9.76142 15.7614 12 13 12C12.5177 12 12.0513 11.9317 11.61 11.8042L9.70711 13.7071C9.51957 13.8946 9.26522 14 9 14H8V15C8 15.5523 7.55228 16 7 16H6V17C6 17.5523 5.55228 18 5 18H3C2.44772 18 2 17.5523 2 17V15C2 14.7348 2.10536 14.4804 2.29289 14.2929L8.19576 8.39003C8.0683 7.94874 8 7.48234 8 7ZM13 4C12.5858 4 12.25 4.33579 12.25 4.75C12.25 5.16421 12.5858 5.5 13 5.5C13.8284 5.5 14.5 6.17157 14.5 7C14.5 7.41421 14.8358 7.75 15.25 7.75C15.6642 7.75 16 7.41421 16 7C16 5.34315 14.6569 4 13 4Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
