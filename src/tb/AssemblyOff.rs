#[cfg(feature = "TbAssemblyOff")]
use leptos::*;
#[cfg(feature = "TbAssemblyOff")]
///This icon requires the feature `TbAssemblyOff` to be enabled.
#[component]
pub fn AssemblyOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-assembly-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.703 4.685l2.326 -1.385a2.056 2.056 0 0 1 2 0l6 3.573h-.029a2 2 0 0 1 1 1.747v6.536c0 .248 -.046 .49 -.132 .715m-2.156 1.837l-4.741 3.029a2 2 0 0 1 -1.942 0l-6 -3.833a2 2 0 0 1 -1.029 -1.747v-6.537a2 2 0 0 1 1.029 -1.748l1.157 -.689"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.593 7.591c.295 -.133 .637 -.12 .921 .04l3 1.79h-.014c.312 .181 .503 .516 .5 .877v1.702m-1.152 2.86l-2.363 1.514a1 1 0 0 1 -.97 0l-3 -1.922a1 1 0 0 1 -.515 -.876v-3.278c0 -.364 .197 -.7 .514 -.877l.568 -.339"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
