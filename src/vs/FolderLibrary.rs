#[cfg(feature = "VsFolderLibrary")]
use leptos::*;
#[cfg(feature = "VsFolderLibrary")]
///This icon requires the feature `VsFolderLibrary` to be enabled.
#[component]
pub fn FolderLibrary(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M7.70996 3H14.5L15.01 3.5V7H14V5.98999H7.68994L6.82996 6.84998L6.47998 7H1.98999V7.48999V11.49V13H7V14H1.51001L1.01001 13.5V6.5V2.5L1.51001 2H6.51001L6.85999 2.15002L7.70996 3ZM7.48999 5H13.99L14 4.01001H7.5L7.14001 3.85999L6.29004 3.01001H2V6.01001H6.28003L7.14001 5.15002L7.48999 5Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "8" y = "8" width = "1" height
        = "6" />< rect xmlns = "http://www.w3.org/2000/svg" x = "10" y = "8" width = "1"
        height = "6" />< rect xmlns = "http://www.w3.org/2000/svg" x = "12.0041" y =
        "8.35193" width = "1" height = "6" transform = "rotate(-20 12.0041 8.35193)" /> <
        title > { title } < / title > < / svg >
    }
}
