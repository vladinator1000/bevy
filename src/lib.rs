//! Bevy is an open-source modular game engine built in Rust, with a focus on developer productivity and performance.
//!
//! Check out the [Bevy website](https://bevyengine.org) for more information, read the 
//! [Bevy Book](https://bevyengine.org/learn/book/introduction) for a step-by-step guide, and [engage with our
//! community](https://bevyengine.org/community/) if you have any questions or ideas!
//!  
//! ```
//!use bevy::prelude::*;
//!
//!fn main() {
//!    App::build()
//!        .add_default_plugins()
//!        .add_system_fn("hello", |_| println!("hello world!"))
//!        .run();
//!}
//! ```
//!
//! The "bevy" crate is just a container crate that makes it easier to consume Bevy components.
//! The defaults provide a "full" engine experience, but you can easily enable / disable features
//! in your project's Cargo.toml to meet your specific needs. See Bevy's Cargo.toml for a full list of features available.

#![feature(min_specialization)]

pub mod prelude;

pub use bevy_app as app;
pub use glam as math;
pub use legion;

#[cfg(feature = "asset")]
pub use bevy_asset as asset;
#[cfg(feature = "core")]
pub use bevy_core as core;
#[cfg(feature = "derive")]
pub use bevy_derive as derive;
#[cfg(feature = "diagnostic")]
pub use bevy_diagnostic as diagnostic;
#[cfg(feature = "input")]
pub use bevy_input as input;
#[cfg(feature = "render")]
pub use bevy_render as render;
#[cfg(feature = "serialization")]
pub use bevy_serialization as serialization;
#[cfg(feature = "transform")]
pub use bevy_transform as transform;
#[cfg(feature = "ui")]
pub use bevy_ui as ui;
#[cfg(feature = "window")]
pub use bevy_window as window;

use app::AppBuilder;

pub trait AddDefaultPlugins {
    fn add_default_plugins(&mut self) -> &mut Self;
}

impl AddDefaultPlugins for AppBuilder {
    fn add_default_plugins(&mut self) -> &mut Self {
        #[cfg(feature = "core")]
        self.add_plugin(bevy_core::CorePlugin::default());

        #[cfg(feature = "input")]
        self.add_plugin(bevy_input::InputPlugin::default());

        #[cfg(feature = "window")]
        self.add_plugin(bevy_window::WindowPlugin::default());

        #[cfg(feature = "render")]
        self.add_plugin(bevy_render::RenderPlugin::default());

        #[cfg(feature = "ui")]
        self.add_plugin(ui::UiPlugin::default());

        #[cfg(feature = "winit")]
        self.add_plugin(bevy_winit::WinitPlugin::default());
        #[cfg(not(feature = "winit"))]
        self.add_plugin(bevy_app::schedule_runner::ScheduleRunnerPlugin::default());

        #[cfg(feature = "wgpu")]
        self.add_plugin(bevy_wgpu::WgpuRendererPlugin::default());

        self
    }
}
