#[cfg(feature = "HiLgSolidMagnifyingGlassMinus")]
use leptos::*;
#[cfg(feature = "HiLgSolidMagnifyingGlassMinus")]
///This icon requires the feature `HiLgSolidMagnifyingGlassMinus` to be enabled.
#[component]
pub fn MagnifyingGlassMinus(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10.5 3.75C6.77208 3.75 3.75 6.77208 3.75 10.5C3.75 14.2279 6.77208 17.25 10.5 17.25C12.3642 17.25 14.0506 16.4953 15.273 15.273C16.4953 14.0506 17.25 12.3642 17.25 10.5C17.25 6.77208 14.2279 3.75 10.5 3.75ZM2.25 10.5C2.25 5.94365 5.94365 2.25 10.5 2.25C15.0563 2.25 18.75 5.94365 18.75 10.5C18.75 12.5078 18.032 14.3491 16.8399 15.7793L21.5303 20.4697C21.8232 20.7626 21.8232 21.2374 21.5303 21.5303C21.2374 21.8232 20.7626 21.8232 20.4697 21.5303L15.7793 16.8399C14.3491 18.032 12.5078 18.75 10.5 18.75C5.94365 18.75 2.25 15.0563 2.25 10.5ZM6.75 10.5C6.75 10.0858 7.08579 9.75 7.5 9.75H13.5C13.9142 9.75 14.25 10.0858 14.25 10.5C14.25 10.9142 13.9142 11.25 13.5 11.25H7.5C7.08579 11.25 6.75 10.9142 6.75 10.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
