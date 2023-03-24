#[cfg(feature = "IoJournal")]
use leptos::*;
#[cfg(feature = "IoJournal")]
///This icon requires the feature `IoJournal` to be enabled.
#[component]
pub fn Journal(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M290,32H144A64.07,64.07,0,0,0,80,96V416a64.07,64.07,0,0,0,64,64H290Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M368,32H350V480h18a64.07,64.07,0,0,0,64-64V96A64.07,64.07,0,0,0,368,32Z" /> <
        title > { title } < / title > < / svg >
    }
}
