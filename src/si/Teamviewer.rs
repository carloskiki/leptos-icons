#[cfg(feature = "SiTeamviewer")]
use leptos::*;
#[cfg(feature = "SiTeamviewer")]
///This icon requires the feature `SiTeamviewer` to be enabled.
#[component]
pub fn Teamviewer(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M22.597 24H1.406A1.41 1.41 0 0 1 0 22.594V1.406A1.41 1.41 0 0 1 1.406 0h21.191a1.41 1.41 0 0 1 1.406 1.406v21.188A1.41 1.41 0 0 1 22.597 24zM11.911 2.109c-5.405.047-9.763 4.482-9.802 9.89-.04 5.507 4.381 9.885 9.89 9.89 5.415.003 9.796-4.5 9.89-9.89.097-5.572-4.406-9.939-9.978-9.89zM9.65 8.633l-.889 2.159H15.3l-.889-2.159 6.642 3.365-6.642 3.365.889-2.159H8.761l.882 2.159-6.659-3.365z"
        /> < title > { title } < / title > < / svg >
    }
}
