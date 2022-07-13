use std::collections::HashMap;
use bevy::prelude::*;
use std::time::Duration;

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
    mut query: Query<(&mut Handle<Image>, &mut Animation)>
) {
    let delta = time.delta();
    for (mut texture, mut animation) in query.iter_mut() {
        animation.update(delta);
        *texture = animation.get_current_texture();
    }
}

fn update_entities_with_animations(
    time: Res<Time>,
    mut query: Query<(&mut Handle<Image>, &mut Animations)>
) {
    let delta = time.delta();
    for (mut texture, mut animations) in query.iter_mut() {
        animations.update(delta);
        *texture = animations.get_current_texture();
    }
}

/// Component that describes a running animation of some entity.
///
/// Basically, you provide a vec of image handles and an animation time. Based
/// on the elapsed time the index of the current texture is determined.
///
/// The animation can be repeatable or not. If it is not repeatable, the last texture
/// in the vector is returned forever.
#[derive(Component, Clone)]
pub struct Animation {
    num_textures: usize,
    current_texture_index: usize,
    duration_secs: f32,
    timer: Timer,
    textures: Vec<Handle<Image>>
}

impl Animation {
    pub fn new<const C: usize>(duration_secs: f32, repeating: bool, textures: [Handle<Image>; C]) -> Self {
        Animation {
            num_textures: textures.len(),
            current_texture_index: 0,
            duration_secs,
            timer: Timer::new(Duration::from_secs_f32(duration_secs), repeating),
            textures: textures.into_iter().collect()
        }
    }

    /// Create an "animation" from a single texture.
    ///
    /// This might be useful if you want to collect multiple animations in an Animations struct,
    /// but some animations only require a single texture.
    pub fn from_single_texture(texture: Handle<Image>) -> Self {
        Animation {
            num_textures: 1,
            current_texture_index: 0,
            duration_secs: 0.0,
            timer: Timer::new(Duration::from_secs_f32(0.0), false),
            textures: vec![texture]
        }
    }

    /// Proceed the timer and calculate the next texture index
    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        let elapsed = self.timer.elapsed_secs();

        match elapsed < self.duration_secs {
            true => {
                let relation = elapsed / self.duration_secs;
                self.current_texture_index = ((self.num_textures as f32) * relation) as usize
            },
            false => self.current_texture_index = self.num_textures - 1
        }
    }

    pub fn get_current_texture(&self) -> Handle<Image> {
        self.textures.get(self.current_texture_index).expect("the current texture index should be in range of the amount of textures").clone()
    }

    /// Rewind the animation back to the start
    pub fn reset(&mut self) {
        self.current_texture_index = 0;
        self.timer.reset()
    }
}

/// Component for entities that might have more than one animation.
///
/// The main advantage of this component is, that every animation is created once and only
/// handles must be cloned.
///
/// The animation can be switched at runtime. Every animation is identified by a string.
#[derive(Component)]
pub struct Animations {
    atlas: HashMap<String, Animation>,
    current: String
}

impl Animations {
    pub fn new<const C: usize, S: ToString>(animations: [(S, Animation); C], current: S) -> Self {
        Animations {
            atlas: animations.into_iter().map(|(s, anims)| (s.to_string(), anims)).collect(),
            current: current.to_string()
        }
    }

    /// Update the currently selected animation
    pub fn update(&mut self, delta: Duration) {
        self.atlas.get_mut(&self.current).expect("current set animation is not part of the animation atlas").update(delta)
    }

    /// Return the current texture just like a single animation.
    pub fn get_current_texture(&self) -> Handle<Image> {
        self.atlas.get(&self.current).expect("current set animation is not part of the animation atlas").get_current_texture()
    }

    /// Change the current animation.
    ///
    /// The newly selected animation gets reset in the process (this also checks if the new name is in the atlas).
    ///
    /// If the new animation is the same as the old animation, nothing is done. This prevents "freezes" when this method
    /// is called very often, every frame for example.
    pub fn change_animation_to(&mut self, animation_name: impl ToString)  {
        let new_current = animation_name.to_string();

        if new_current != self.current {
            self.atlas.get_mut(&new_current).expect("the new selected animation does not exist in the atlas").reset();
            self.current = new_current;
        }
    }
}