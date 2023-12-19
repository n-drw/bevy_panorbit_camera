use bevy::prelude::*;

/// A resource that tracks whether egui wants focus on the current and previous frames.
///
/// The reason the previous frame's value is saved is because when you click inside an
/// egui window, Context::wants_pointer_input() still returns false once before returning
/// true. If the camera stops taking input only when it returns false, there's one frame
/// where both egui and the camera are using the input events, which is not desirable.
///
/// This is re-exported in case it's useful. I recommend only using input events if both
/// `prev` and `curr` are false.
#[derive(Resource, PartialEq, Eq, Default)]
pub struct EguiWantsFocus {
    /// Whether egui wanted focus on the previous frame
    pub prev: bool,
    /// Whether egui wants focus on the current frame
    pub curr: bool,
}

pub fn check_egui_wants_focus(
    mut contexts: bevy_egui::EguiContexts,
    mut wants_focus: ResMut<EguiWantsFocus>,
    windows: Query<Entity, With<Window>>,
) {
    // If _any_ egui context wants focus, then we treat that as 'egui wants focus', because only
    // one window can receive input events at a time anyway. In other words, there's no need to
    // match the egui context with the window currently receiving events.
    let new_wants_focus = windows.iter().any(|window| {
        let ctx = contexts.ctx_for_window_mut(window);
        ctx.wants_pointer_input() || ctx.wants_keyboard_input()
    });
    let new_res = EguiWantsFocus {
        prev: wants_focus.curr,
        curr: new_wants_focus,
    };
    wants_focus.set_if_neq(new_res);
}
