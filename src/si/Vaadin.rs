#[cfg(feature = "SiVaadin")]
use leptos::*;
#[cfg(feature = "SiVaadin")]
///This icon requires the feature `SiVaadin` to be enabled.
#[component]
pub fn Vaadin(
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
        "M1.166.521C.506.521 0 1.055 0 1.715v1.97c0 2.316 1.054 3.473 3.502 3.473h5.43c1.623 0 1.783.685 1.783 1.35 0 .068.004.13.012.193a1.268 1.268 0 0 0 2.531-.004c.007-.062.012-.121.012-.19 0-.664.16-1.349 1.783-1.349h5.43C22.93 7.158 24 6.001 24 3.686V1.715c0-.66-.524-1.194-1.184-1.194-.66 0-1.189.534-1.189 1.194l-.004.685c0 .746-.476 1.27-1.594 1.27h-5.322c-2.422 0-2.608 1.796-2.687 2.748h-.055c-.08-.952-.266-2.748-2.688-2.748H3.955c-1.118 0-1.629-.544-1.629-1.29v-.665c0-.66-.5-1.194-1.16-1.194zm5.875 10.553a1.586 1.586 0 0 0-1.375 2.371c1.657 3.06 3.308 6.13 4.967 9.184a1.415 1.415 0 0 0 2.586.02l.033-.06 4.945-9.142a1.587 1.587 0 0 0-1.377-2.373c-.702 0-1.179.345-1.502 1.082l-3.386 6.313-3.383-6.305c-.326-.745-.805-1.09-1.508-1.09Z"
        /> < title > { title } < / title > < / svg >
    }
}
