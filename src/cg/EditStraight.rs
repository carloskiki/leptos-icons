#[cfg(feature = "CgEditStraight")]
use leptos::*;
#[cfg(feature = "CgEditStraight")]
///This icon requires the feature `CgEditStraight` to be enabled.
#[component]
pub fn EditStraight(
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
        "M12 4C15.866 4 19 7.13401 19 11H5C5 7.13401 8.13401 4 12 4Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 13H1V11H5V13Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 13C19 16.866 15.866 20 12 20C8.13401 20 5 16.866 5 13H19Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 13V11H23V13H19Z" fill = "currentColor" /> < title > { title } < / title > <
        / svg >
    }
}
