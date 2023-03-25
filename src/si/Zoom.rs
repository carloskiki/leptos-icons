#[cfg(feature = "SiZoom")]
use leptos::*;
#[cfg(feature = "SiZoom")]
///This icon requires the feature `SiZoom` to be enabled.
#[component]
pub fn Zoom(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.585 13.607l-.27-.012H1.886l3.236-3.237-.013-.27a.815.815 0 00-.796-.796l-.27-.013H0l.014.27c.034.438.353.77.794.796l.27.013h2.43L.268 13.595l.014.269c.015.433.362.78.795.796l.27.013h4.046l-.014-.27c-.036-.443-.35-.767-.795-.795zm3.238-4.328h-.004a2.696 2.697 0 10.003 0zm1.141 3.841a1.619 1.619 0 11-2.289-2.288 1.619 1.619 0 012.289 2.288zM21.84 9.28a2.158 2.158 0 00-1.615.73 2.153 2.153 0 00-1.619-.732 2.148 2.148 0 00-1.208.37c-.21-.233-.68-.37-.949-.37v5.395l.27-.013c.45-.03.778-.349.796-.796l.013-.27v-1.889l.014-.27c.01-.202.04-.382.132-.54a1.078 1.079 0 011.473-.393 1.078 1.079 0 01.393.392c.093.16.12.34.132.54l.014.271v1.889l.013.269a.83.83 0 00.795.796l.27.013v-2.967l.012-.27c.01-.2.04-.384.134-.543.3-.514.96-.69 1.473-.39a1.078 1.079 0 01.393.393c.092.16.12.343.13.54l.015.27v1.889l.013.269c.028.443.35.77.796.796l.27.013v-3.237a2.158 2.158 0 00-2.16-2.156zm-10.263.788a2.697 2.698 0 103.811 3.816 2.697 2.698 0 00-3.811-3.816zm3.05 3.052a1.619 1.619 0 11-2.289-2.29 1.619 1.619 0 012.289 2.29z"
        /> < title > { title } < / title > < / svg >
    }
}
