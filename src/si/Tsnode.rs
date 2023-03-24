#[cfg(feature = "SiTsnode")]
use leptos::*;
#[cfg(feature = "SiTsnode")]
///This icon requires the feature `SiTsnode` to be enabled.
#[component]
pub fn Tsnode(
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
        "M11.999 0c-.196 0-.392.05-.568.153L2.026 5.58a1.135 1.135 0 00-.568.983V17.43c0 .406.216.781.568.984l5.787 3.344v-7.344H4.748v-1.943h8.342v1.943h-3.065v8.622l1.406.812c.351.203.784.203 1.136 0l2.317-1.338a3.958 3.958 0 01-1.195-1.413l1.801-1.042c.361.59.806 1.06 1.48 1.25l2.174-1.256c-.127-.568-.698-.823-1.584-1.21l-.553-.238c-1.596-.68-2.655-1.532-2.655-3.334 0-1.658 1.265-2.922 3.24-2.922 1.406 0 2.417.49 3.144 1.77l-1.723 1.105c-.379-.68-.79-.948-1.421-.948-.648 0-1.06.41-1.06.948 0 .663.412.932 1.36 1.343l.553.237c1.336.573 2.255 1.155 2.676 2.107l.853-.493c.352-.203.568-.578.568-.984V6.565c0-.406-.216-.782-.568-.984L12.567.153A1.134 1.134 0 0011.999 0z"
        /> < title > { title } < / title > < / svg >
    }
}
