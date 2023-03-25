#[cfg(feature = "SiZcash")]
use leptos::*;
#[cfg(feature = "SiZcash")]
///This icon requires the feature `SiZcash` to be enabled.
#[component]
pub fn Zcash(
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
        "M12 0C5.382 0 0 5.382 0 12s5.382 12 12 12 12-5.382 12-12S18.618 0 12 0zm0 22.02C6.474 22.02 1.98 17.526 1.98 12S6.474 1.98 12 1.98 22.02 6.474 22.02 12 17.526 22.02 12 22.02zm4.28-13.763V6.431h-3.274V4.42h-2.012V6.43H7.72v2.423h5.079l-5.08 6.889v1.826h3.275v2.002h2.012v-2.002h3.275v-2.422H11.2z"
        /> < title > { title } < / title > < / svg >
    }
}
