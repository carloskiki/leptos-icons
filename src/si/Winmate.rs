#[cfg(feature = "SiWinmate")]
use leptos::*;
#[cfg(feature = "SiWinmate")]
///This icon requires the feature `SiWinmate` to be enabled.
#[component]
pub fn Winmate(
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
        "M5.785 4.058l-4.473.004L1.311.01 19.469 0c3.514.42 3.199 4.047 3.199 4.047l-2.156-.002-2.769 15.888L14.79 4.049l-4.731.005.856 7.376-2.137 8.507L5.785 4.058zM4.491 21.373L1.317 8.52l.009 12.338C1.756 23.983 4.629 24 4.629 24l1.687-.001c-1.393-.69-1.825-2.626-1.825-2.626zm9.237.659l-1.724-6.724-1.673 6.678c-.517 1.652-1.702 2.009-1.702 2.009l6.602-.002c-1.206-.499-1.503-1.961-1.503-1.961zm8.949-17.643l-2.844 15.865c-.711 3.767-2.285 3.738-2.285 3.738l5.141-.008-.012-19.595z"
        /> < title > { title } < / title > < / svg >
    }
}
