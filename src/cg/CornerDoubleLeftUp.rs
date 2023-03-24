#[cfg(feature = "CgCornerDoubleLeftUp")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleLeftUp")]
///This icon requires the feature `CgCornerDoubleLeftUp` to be enabled.
#[component]
pub fn CornerDoubleLeftUp(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.78372 9.25045L6.36951 7.83624L10.6121 3.59363L14.8548 7.83627L13.4406 9.25048L10.6121 6.42205L7.78372 9.25045Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.4406 13.4932L14.8548 12.0789L10.6121 7.83628L6.36951 12.0789L7.78372 13.4931L9.63052 11.6463V16.4063C9.63052 18.6155 11.4214 20.4063 13.6305 20.4063H17.6305V18.4063H13.6305C12.526 18.4063 11.6305 17.5109 11.6305 16.4063V11.6831L13.4406 13.4932Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
