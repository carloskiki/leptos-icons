#[cfg(feature = "BsFiletypeMp4")]
use leptos::*;
#[cfg(feature = "BsFiletypeMp4")]
///This icon requires the feature `BsFiletypeMp4` to be enabled.
#[component]
pub fn FiletypeMp4(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-filetype-mp4" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" d =
        "M14 4.5V14a2 2 0 0 1-2 2v-1a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM.706 15.849v-2.66h.038l.952 2.16h.516l.946-2.16h.038v2.66h.715V11.85h-.8l-1.14 2.596h-.026L.805 11.85H0v3.999h.706Zm5.278-3.999h-1.6v3.999h.792v-1.342h.803c.287 0 .53-.057.732-.173.203-.117.357-.275.463-.474a1.42 1.42 0 0 0 .161-.677c0-.25-.053-.476-.158-.677a1.176 1.176 0 0 0-.46-.477 1.4 1.4 0 0 0-.733-.179Zm.545 1.333a.795.795 0 0 1-.085.38.574.574 0 0 1-.237.241.794.794 0 0 1-.375.082h-.66V12.48h.66c.219 0 .39.06.513.181.123.122.184.296.184.522Zm1.505-.032c.266-.434.53-.867.791-1.301h1.14v2.62h.49v.638h-.49v.741h-.741v-.741H7.287v-.648c.235-.44.484-.876.747-1.31Zm-.029 1.298v.02h1.219v-2.021h-.041c-.201.318-.404.646-.607.984-.2.338-.391.677-.571 1.017Z"
        /> < title > { title } < / title > < / svg >
    }
}
