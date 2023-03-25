#[cfg(feature = "OcLgIssueTracks")]
use leptos::*;
#[cfg(feature = "OcLgIssueTracks")]
///This icon requires the feature `OcLgIssueTracks` to be enabled.
#[component]
pub fn IssueTracks(
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
        "M2.5 12a9.5 9.5 0 1 1 19 0 .75.75 0 0 0 1.5 0c0-6.075-4.925-11-11-11S1 5.925 1 12s4.925 11 11 11a.75.75 0 0 0 0-1.5A9.5 9.5 0 0 1 2.5 12Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Zm2.5 2.75a.75.75 0 0 1 .75-.75h7a.75.75 0 0 1 0 1.5h-7a.75.75 0 0 1-.75-.75Zm3.75 2.75a.75.75 0 0 0 0 1.5h4a.75.75 0 0 0 0-1.5h-4Z"
        /> < title > { title } < / title > < / svg >
    }
}
