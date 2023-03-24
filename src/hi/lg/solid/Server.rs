#[cfg(feature = "HiLgSolidServer")]
use leptos::*;
#[cfg(feature = "HiLgSolidServer")]
///This icon requires the feature `HiLgSolidServer` to be enabled.
#[component]
pub fn Server(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.07993 5.22701C4.43013 3.91375 5.61948 3 6.97863 3H17.0214C18.3805 3 19.5699 3.91375 19.9201 5.22701L22.0338 13.1535C21.1346 12.4318 19.9927 12 18.75 12H5.25C4.00727 12 2.86538 12.4318 1.96619 13.1535L4.07993 5.22701Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M5.25 13.5C3.17893 13.5 1.5 15.1789 1.5 17.25C1.5 19.3211 3.17893 21 5.25 21H18.75C20.8211 21 22.5 19.3211 22.5 17.25C22.5 15.1789 20.8211 13.5 18.75 13.5H5.25ZM15.75 18C16.1642 18 16.5 17.6642 16.5 17.25C16.5 16.8358 16.1642 16.5 15.75 16.5C15.3358 16.5 15 16.8358 15 17.25C15 17.6642 15.3358 18 15.75 18ZM19.5 17.25C19.5 17.6642 19.1642 18 18.75 18C18.3358 18 18 17.6642 18 17.25C18 16.8358 18.3358 16.5 18.75 16.5C19.1642 16.5 19.5 16.8358 19.5 17.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
