#[cfg(feature = "SiTeamcity")]
use leptos::*;
#[cfg(feature = "SiTeamcity")]
///This icon requires the feature `SiTeamcity` to be enabled.
#[component]
pub fn Teamcity(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "m0 0v24h24v-24zm2.664 2.964h7.48v1.832h-2.748v7.196h-1.984v-7.196h-2.748zm9.328 18h-9v-1.5h9zm5.56391-9.21826a4.62 4.62 0 0 1 -2.03591.37426 4.556 4.556 0 0 1 -4.628-4.616v-.024a4.584 4.584 0 0 1 4.708-4.668 4.65561 4.65561 0 0 1 3.56 1.388l-1.264 1.456a3.33594 3.33594 0 0 0 -2.312-1.02 2.67116 2.67116 0 0 0 -2.616 2.8v.028a2.68058 2.68058 0 0 0 2.616 2.836 3.22606 3.22606 0 0 0 2.376-1.056l1.264 1.276a4.61947 4.61947 0 0 1 -1.66809 1.22573z"
        /> < title > { title } < / title > < / svg >
    }
}
