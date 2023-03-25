#[cfg(feature = "SiLeanpub")]
use leptos::*;
#[cfg(feature = "SiLeanpub")]
///This icon requires the feature `SiLeanpub` to be enabled.
#[component]
pub fn Leanpub(
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
        "M22.19 5.284c-.806-.454-2.93-1.478-5.652-1.478-2.445 0-3.837.751-4.538 1.234-.701-.483-2.093-1.235-4.538-1.235-2.723 0-4.846 1.025-5.651 1.479L0 20.194h.054a6.933 6.933 0 002.957-.673c1.032-.487 2.66-1.06 4.602-1.06 2.364 0 3.71 1.056 4.387 1.733.678-.677 2.023-1.732 4.387-1.732 1.943 0 3.57.572 4.602 1.06a6.933 6.933 0 002.957.672H24zM20.934 17.78a12.167 12.167 0 00-2.875-.801c-.558-.081-1.12-.122-1.674-.122-1.571 0-2.991.392-4.22 1.165l-.166.103-.165-.103c-1.23-.773-2.65-1.165-4.222-1.165-.552 0-1.115.04-1.673.122-.949.137-1.916.407-2.875.801l-1.218.501L3.3 6.321l.108-.048c1.225-.542 2.797-.865 4.205-.865 1.745 0 3.22.556 4.387 1.652 1.168-1.096 2.642-1.652 4.386-1.652 1.409 0 2.98.323 4.206.865l.108.048 1.453 11.961zm-4.085-1.328a11.99 11.99 0 00-.464-.009c-1.627 0-3.103.402-4.386 1.194-1.283-.792-2.759-1.194-4.387-1.194-.572 0-1.155.043-1.732.126-.983.143-1.983.421-2.973.829l-.565.232 1.34-11.025c1.159-.492 2.618-.783 3.93-.783 1.705 0 3.131.562 4.24 1.672L12 7.64l.146-.146c1.07-1.07 2.433-1.629 4.055-1.669z"
        /> < title > { title } < / title > < / svg >
    }
}
