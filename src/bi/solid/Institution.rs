#[cfg(feature = "BiSolidInstitution")]
use leptos::*;
#[cfg(feature = "BiSolidInstitution")]
///This icon requires the feature `BiSolidInstitution` to be enabled.
#[component]
pub fn Institution(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m21.857 8.485-3-5A.997.997 0 0 0 18 3h-4.586l-.707-.707a.999.999 0 0 0-1.414 0L10.586 3H6a.997.997 0 0 0-.857.485l-3 5A1.001 1.001 0 0 0 2.002 9H2v10a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1V9h-.002c0-.178-.046-.356-.141-.515zM20 18h-6v-4h-4v4H4v-8h2.414l.293-.293 2-2L12 4.414l4.293 4.293 1 1 .293.293H20v8z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "11.895" cy = "9.895" r =
        "2.105" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 12h2v3H6zm10 0h2v3h-2z" /> < title > { title } < / title > < / svg >
    }
}
