#[cfg(feature = "SiEclipsevertdotx")]
use leptos::*;
#[cfg(feature = "SiEclipsevertdotx")]
///This icon requires the feature `SiEclipsevertdotx` to be enabled.
#[component]
pub fn Eclipsevertdotx(
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
        "M3.356.01C1.566.01.027 1.269 0 2.938v1.436h2.515l3.861 8.896 4.028-8.791h5.078l2.182 3.986 2.56-3.986H24V2.946C24 1.281 22.44.011 20.645.011zM24 5.668l-8.874 13.56H12.44c-.02-.629-.188-1.237-.503-1.74l3.609-5.708-2.744-4.36-3.829 8.42-.036-.002a3.443 3.443 0 0 0-3.434 3.433c0 .021.003.042.004.063h-.263L0 7.5v13.553c0 1.665 1.56 2.935 3.356 2.935h17.289c1.812 0 3.355-1.276 3.355-2.935v-1.826h-3.587l-1.594-2.874 2.224-3.378L24 17.638zm-15.066 11.5a2.102 2.102 0 0 1 2.109 2.103 2.106 2.106 0 1 1-4.212 0c0-1.16.937-2.1 2.103-2.103Z"
        /> < title > { title } < / title > < / svg >
    }
}
