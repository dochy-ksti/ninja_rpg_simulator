#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

use std::path::Path;
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::{Texture, TextureContext};


mod imp;
#[allow(dead_code)]
mod testing;
type PistonGlyph<'a> = GlyphCache<'a, TextureContext<gfx_device_gl::Factory, gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>, Texture<gfx_device_gl::Resources>>;

pub use imp::structs::gui_item::{GuiItem, GuiItems};
pub use imp::structs::gui_output::GuiOutput;

pub use crate::imp::start_loop::start_loop;
pub use crate::testing::test_open_window::test_open_window;