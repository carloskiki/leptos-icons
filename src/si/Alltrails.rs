#[cfg(feature = "SiAlltrails")]
use leptos::*;
#[cfg(feature = "SiAlltrails")]
///This icon requires the feature `SiAlltrails` to be enabled.
#[component]
pub fn Alltrails(
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
        "M23.694 18.72c-3.845-4.058-12.083-5.754-17.345-3.216l5.504-7.623 2.63 4.163c.148.246.47.32.71.164l1.993-1.333c1.684 1.806 3.358 3.618 4.796 5.158a.19.19 0 00.322-.139.185.185 0 00-.023-.088h.001c-1.41-2.458-2.826-4.913-4.25-7.364-.254-.434-.552-.503-.986-.219l-1.34.921-3.382-5.226a.587.587 0 00-.494-.275.574.574 0 00-.487.265A5684.859 5684.859 0 01.08 18.973v.002c-.2.28-.001.67.343.671.175-.002.331-.124.488-.2 3.142-1.626 10.197-3.996 15.073-.26 2.141 1.817 9.61 1.408 7.71-.465"
        /> < title > { title } < / title > < / svg >
    }
}
