use std::collections::HashMap;
use bevy::prelude::*;
use std::time::Duration;
use crate::spritesheet::SpriteSheet;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_entities_with_single_animation)
            .add_system(update_entities_with_animations)
        ;
    }
}

fn update_entities_with_single_animation(
    time: Res<Time>,
    sheets: Res<Assets<SpriteSheet>>,
    mut query: Query<(&mut Handle<Image>, &mut Animation)>,
) {
    let delta = time.delta();
    for (mut texture, mut animation) in query.iter_mut() {
        animation.update(delta);

        if let Some(a) = animation.get_current_texture(&sheets) {
            *texture = a
        }
    }
}

fn update_entities_with_animations(
    time: Res<Time>,
    sheets: Res<Assets<SpriteSheet>>,
    mut query: Query<(&mut Handle<Image>, &mut Animations)>,
) {
    let delta = time.delta();
    for (mut texture, mut animations) in query.iter_mut() {
        animations.current_mut().update(delta);

        if let Some(a) = animations.current().get_current_texture(&sheets) {
            *texture = a
        }
    }
}

/// Component that describes a running animation of some entity.
///
/// Basically, you provide a vec of image handles and an animation time. Based
/// on the elapsed time the index of the current texture is determined.
///
/// The animation can be repeatable or not. If it is not repeatable, the last texture
/// in the vector is returned forever.
///
/// TODO: add the ability to check if an animation changed
#[derive(Component, Clone)]
pub enum Animation {
    SingleTexture {
        texture: Handle<Image>
    },
    TextureList {
        current_texture_index: usize,
        timer: Timer,
        repeating: bool,
        textures: Vec<Handle<Image>>,
        running: bool,
    },
    SpriteSheet {
        current_texture_index: usize,
        timer: Timer,
        repeating: bool,
        num_textures: usize,
        sheet: Handle<SpriteSheet>,
        running: bool,
    },
}

impl Animation {
    /// Create an animation from a single texture. This will always return the same handle when get_current_texture is called. No timers or indexes get updated.
    ///
    /// Why? To be used in an Animations collection where some states aren't animated (like the eyes of an eaten ghost).
    pub fn from_texture(texture: Handle<Image>) -> Self {
        Animation::SingleTexture { texture }
    }

    /// Create an animation from an iterator of image handles.
    pub fn from_textures(duration_secs: f32, repeating: bool, textures: impl IntoIterator<Item=Handle<Image>>) -> Self {
        let textures = textures
            .into_iter()
            .collect::<Vec<_>>();
        let texture_display_time = duration_secs / textures.len() as f32;

        Animation::TextureList {
            current_texture_index: 0,
            timer: Timer::new(Duration::from_secs_f32(texture_display_time), true),
            repeating,
            textures,
            running: true,
        }
    }

    /// Create an animation from a sprite sheet handle.
    ///
    /// Only the sheet handle is stored. When get_current_texture is called, the current texture handle is cloned from the sheet.
    ///
    /// TODO: maybe cache the current handle or at least prevent Assets access when the animation did not change
    /// TODO: num_textures must be provided currently, as this information is available after the sheet loaded. But it might be possible to start without it and update later
    pub fn from_sprite_sheet(duration_secs: f32, repeating: bool, num_textures: usize, sheet: Handle<SpriteSheet>) -> Self {
        let texture_display_time = duration_secs / num_textures as f32;
        Animation::SpriteSheet {
            current_texture_index: 0,
            timer: Timer::new(Duration::from_secs_f32(texture_display_time), true),
            repeating,
            num_textures,
            sheet,
            running: true,
        }
    }

    /// Update the animation.
    ///
    /// For texture lists and sprite sheet, this process is mostly the same: The timer for the current sprite gets
    /// updated with the given delta.
    /// If the timer finished, increase the current texture index.
    /// But if the index is at its max, set it to zero if the animation does not repeat.
    ///
    /// If the animation is stopped or it is a single texture animation, do nothing.
    pub fn update(&mut self, delta: Duration) {
        let running = match self {
            Animation::SingleTexture { .. } => return,
            Animation::TextureList { running, .. } => *running,
            Animation::SpriteSheet { running, .. } => *running
        };

        if !running { return; }

        let (current_texture_index, timer, repeating, num_textures) = match self {
            Animation::SingleTexture { .. } => return,
            Animation::TextureList { ref mut current_texture_index, timer, repeating, textures, .. } => (current_texture_index, timer, repeating, textures.len()),
            Animation::SpriteSheet { ref mut current_texture_index, timer, repeating, num_textures, .. } => (current_texture_index, timer, repeating, *num_textures)
        };

        timer.tick(delta);

        if timer.just_finished() {
            let at_last_index = *current_texture_index == num_textures - 1;
            match (repeating, at_last_index) {
                (true, true) => *current_texture_index = 0,
                (_, false) => *current_texture_index += 1,
                (false, true) => ()
            }
        }
    }

    pub fn get_current_texture(&self, sheets: &Assets<SpriteSheet>) -> Option<Handle<Image>> {
        match self {
            Animation::SingleTexture { texture } => Some(texture.clone()),
            Animation::TextureList { current_texture_index, textures, .. } => textures.get(*current_texture_index).map(Clone::clone),
            Animation::SpriteSheet { current_texture_index, sheet, .. } => sheets.get(sheet)?.textures.get(*current_texture_index).map(Clone::clone)
        }
    }

    /// Rewind the animation back to the start. This means:
    /// - reset the timer
    /// - set the current texture index to zero
    /// - set running to true
    pub fn reset(&mut self) {
        let (current_texture_index, timer, running) = match self {
            Animation::SingleTexture { .. } => return,
            Animation::TextureList { ref mut current_texture_index, timer, running, .. } => (current_texture_index, timer, running),
            Animation::SpriteSheet { ref mut current_texture_index, timer, running, .. } => (current_texture_index, timer, running)
        };

        timer.reset();
        *current_texture_index = 0;
        *running = true;
    }

    /// Return if the current animation iteration is over
    pub fn is_finished(&self) -> bool {
        let (current_texture_index, timer, num_textures) = match self {
            Animation::SingleTexture { .. } => return true,
            Animation::TextureList { current_texture_index, timer, textures, .. } => (current_texture_index, timer, textures.len()),
            Animation::SpriteSheet { current_texture_index, timer, num_textures, .. } => (current_texture_index, timer, *num_textures)
        };

        *current_texture_index == num_textures - 1 && timer.just_finished()
    }

    /// Return if an animation is completely finished, not just the current iteration
    pub fn is_completely_finished(&self) -> bool {
        let repeating = match self {
            Animation::SingleTexture { .. } => false,
            Animation::TextureList { repeating, .. } => *repeating,
            Animation::SpriteSheet { repeating, .. } => *repeating
        };

        !repeating && self.is_finished()
    }

    /// Stop the animation from getting updated.
    pub fn stop(&mut self) {
        match self {
            Animation::SingleTexture { .. } => return,
            Animation::TextureList { ref mut running, .. } => *running = false,
            Animation::SpriteSheet { ref mut running, .. } => *running = false,
        }
    }
}

/// Component for entities that might have more than one animation.
///
/// The main advantage of this component is, that every animation is created once and only
/// handles must be cloned.
///
/// The animation can be switched at runtime. Every animation is identified by a string.
///
/// TODO: We could need an Animations variant with a shared timer between animations.
///  The ghosts currently use all the same timer duration, and the animation transition could look
///  smoother this way
#[derive(Component)]
pub struct Animations {
    atlas: HashMap<String, Animation>,
    current: String,
}

impl Animations {
    pub fn new<C: ToString, S: ToString>(animations: impl IntoIterator<Item=(S, Animation)>, current: C) -> Self {
        Animations {
            atlas: animations.into_iter().map(|(s, anims)| (s.to_string(), anims)).collect(),
            current: current.to_string(),
        }
    }

    pub fn current(&self) -> &Animation {
        self.atlas.get(&self.current).expect("current set animation is not part of the animation atlas")
    }

    pub fn current_mut(&mut self) -> &mut Animation {
        self.atlas.get_mut(&self.current).expect("current set animation is not part of the animation atlas")
    }

    /// Change the current animation.
    ///
    /// The newly selected animation gets reset in the process (this also checks if the new name is in the atlas).
    ///
    /// If the new animation is the same as the old animation, nothing is done. This prevents "freezes" when this method
    /// is called very often, every frame for example.
    pub fn change_animation_to(&mut self, animation_name: impl ToString) {
        let new_current = animation_name.to_string();

        if new_current != self.current {
            self.atlas.get_mut(&new_current).expect("the new selected animation does not exist in the atlas").reset();
            self.current = new_current;
        }
    }
}