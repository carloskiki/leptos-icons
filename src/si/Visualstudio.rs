#[cfg(feature = "SiVisualstudio")]
use leptos::*;
#[cfg(feature = "SiVisualstudio")]
///This icon requires the feature `SiVisualstudio` to be enabled.
#[component]
pub fn Visualstudio(
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
        "M17.583.063a1.5 1.5 0 00-1.032.392 1.5 1.5 0 00-.001 0A.88.88 0 0016.5.5L8.528 9.316 3.875 5.5l-.407-.35a1 1 0 00-1.024-.154 1 1 0 00-.012.005l-1.817.75a1 1 0 00-.077.036 1 1 0 00-.047.028 1 1 0 00-.038.022 1 1 0 00-.048.034 1 1 0 00-.03.024 1 1 0 00-.044.036 1 1 0 00-.036.033 1 1 0 00-.032.035 1 1 0 00-.033.038 1 1 0 00-.035.044 1 1 0 00-.024.034 1 1 0 00-.032.05 1 1 0 00-.02.035 1 1 0 00-.024.05 1 1 0 00-.02.045 1 1 0 00-.016.044 1 1 0 00-.016.047 1 1 0 00-.015.055 1 1 0 00-.01.04 1 1 0 00-.008.054 1 1 0 00-.006.05A1 1 0 000 6.668v10.666a1 1 0 00.615.917l1.817.764a1 1 0 001.035-.164l.408-.35 4.653-3.815 7.973 8.815a1.5 1.5 0 00.072.065 1.5 1.5 0 00.057.05 1.5 1.5 0 00.058.042 1.5 1.5 0 00.063.044 1.5 1.5 0 00.065.038 1.5 1.5 0 00.065.036 1.5 1.5 0 00.068.031 1.5 1.5 0 00.07.03 1.5 1.5 0 00.073.025 1.5 1.5 0 00.066.02 1.5 1.5 0 00.08.02 1.5 1.5 0 00.068.014 1.5 1.5 0 00.075.01 1.5 1.5 0 00.075.008 1.5 1.5 0 00.073.003 1.5 1.5 0 00.077 0 1.5 1.5 0 00.078-.005 1.5 1.5 0 00.067-.007 1.5 1.5 0 00.087-.015 1.5 1.5 0 00.06-.012 1.5 1.5 0 00.08-.022 1.5 1.5 0 00.068-.02 1.5 1.5 0 00.07-.028 1.5 1.5 0 00.09-.037l4.944-2.377a1.5 1.5 0 00.476-.362 1.5 1.5 0 00.09-.112 1.5 1.5 0 00.004-.007 1.5 1.5 0 00.08-.125 1.5 1.5 0 00.062-.12 1.5 1.5 0 00.009-.017 1.5 1.5 0 00.04-.108 1.5 1.5 0 00.015-.037 1.5 1.5 0 00.03-.107 1.5 1.5 0 00.009-.037 1.5 1.5 0 00.017-.1 1.5 1.5 0 00.008-.05 1.5 1.5 0 00.006-.09 1.5 1.5 0 00.004-.08V3.942a1.5 1.5 0 000-.003 1.5 1.5 0 000-.032 1.5 1.5 0 00-.01-.15 1.5 1.5 0 00-.84-1.17L18.206.21a1.5 1.5 0 00-.622-.146zM18 6.92v10.163l-6.198-5.08zM3 8.574l3.099 3.427-3.1 3.426z"
        /> < title > { title } < / title > < / svg >
    }
}
