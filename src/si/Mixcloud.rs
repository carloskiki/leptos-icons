#[cfg(feature = "SiMixcloud")]
use leptos::*;
#[cfg(feature = "SiMixcloud")]
///This icon requires the feature `SiMixcloud` to be enabled.
#[component]
pub fn Mixcloud(
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
        "M2.462 8.596l1.372 6.49h.319l1.372-6.49h2.462v6.808H6.742v-5.68l.232-.81h-.402l-1.43 6.49H2.854l-1.44-6.49h-.391l.222.81v5.68H0V8.596zM24 8.63v1.429L21.257 12 24 13.941v1.43l-3.235-2.329h-.348l-3.226 2.329v-1.43l2.734-1.94-2.733-1.942V8.63l3.225 2.338h.348zm-7.869 2.75v1.24H9.304v-1.24z"
        /> < title > { title } < / title > < / svg >
    }
}
