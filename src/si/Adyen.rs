#[cfg(feature = "SiAdyen")]
use leptos::*;
#[cfg(feature = "SiAdyen")]
///This icon requires the feature `SiAdyen` to be enabled.
#[component]
pub fn Adyen(
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
        "M11.64703 9.88245v2.93377c0 .13405.10867.24271.24272.24271h.46316V9.88245h1.76474v5.1503c0 .46916-.38033.8495-.8495.8495H9.94303v-1.23507h2.40991v-.52942h-1.62108c-.46917 0-.8495-.38033-.8495-.8495V9.88245h1.76467Zm-8.26124.00001c.46917 0 .8495.38034.8495.8495v3.3858H.8495c-.46916 0-.8495-.38033-.8495-.8495v-.94805c0-.46917.38034-.8495.8495-.8495h.91521v1.3455c0 .13406.10867.24272.24272.24272h.46316V11.184c0-.13405-.10867-.24271-.24272-.24271l-2.16719-.00002V9.88246Zm5.79068-1.76471v6.00001H5.79068c-.46917 0-.8495-.38033-.8495-.8495v-2.53631c0-.46917.38033-.8495.8495-.8495h.91515v2.93377c0 .13405.10867.24271.24272.24271h.46316l.00005-4.94118h1.76471Zm9.03286 1.76471a.8495.8495 0 0 1 .8495.8495v.94805c0 .46917-.38033.8495-.8495.8495h-.9152v-1.3455c0-.13404-.10868-.2427-.24272-.2427h-.46317v1.8749c0 .13406.10867.24272.24272.24272h2.16719v1.05883h-3.32511c-.46917 0-.8495-.38033-.8495-.8495v-3.3858Zm4.94117 0c.46916 0 .8495.38034.8495.8495v3.3858h-1.7647V11.184c-.0004-.13388-.10884-.24232-.24272-.24272h-.46316v3.1765H19.7647V9.88245Z"
        /> < title > { title } < / title > < / svg >
    }
}
