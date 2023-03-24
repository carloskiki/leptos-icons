#[cfg(feature = "SiBigbluebutton")]
use leptos::*;
#[cfg(feature = "SiBigbluebutton")]
///This icon requires the feature `SiBigbluebutton` to be enabled.
#[component]
pub fn Bigbluebutton(
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
        "M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zM6.838 4.516c.743 0 1.378.364 1.904 1.091.526.728.787 1.602.787 2.625v6.76c0 .539.27.809.809.809h4.174c.538 0 .808-.27.808-.809v-3.205c0-.52-.27-.788-.808-.807h-.807c-1.041-.036-1.923-.308-2.64-.816-.719-.507-1.077-1.133-1.077-1.877h4.524c.97 0 1.796.342 2.478 1.024a3.374 3.374 0 0 1 1.024 2.476v3.205c0 .97-.342 1.797-1.024 2.479-.682.682-1.509 1.021-2.478 1.021h-4.174c-.97 0-1.795-.339-2.477-1.021a3.376 3.376 0 0 1-1.023-2.479V4.516Z"
        /> < title > { title } < / title > < / svg >
    }
}
