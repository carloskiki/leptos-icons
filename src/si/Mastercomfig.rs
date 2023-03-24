#[cfg(feature = "SiMastercomfig")]
use leptos::*;
#[cfg(feature = "SiMastercomfig")]
///This icon requires the feature `SiMastercomfig` to be enabled.
#[component]
pub fn Mastercomfig(
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
        "M12 0C5.479 0 .174 5.304.174 11.826V24h1.337v-6.716C3.486 21.064 7.446 23.65 12 23.65c4.554 0 8.514-2.586 10.49-6.367V24h1.336V11.826h-1.337c0 5.798-4.69 10.489-10.489 10.489-5.798 0-10.49-4.691-10.49-10.49C1.51 6.028 6.203 1.338 12 1.338zm0 3.72a8.107 8.107 0 100 16.214 8.107 8.107 0 000-16.215zm0 1.336a6.77 6.77 0 110 13.538 6.77 6.77 0 010-13.538z"
        /> < title > { title } < / title > < / svg >
    }
}
