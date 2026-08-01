use crate::event::MouseScrollDelta;

pub(super) fn android_mouse_scroll_delta(
    horizontal: f32,
    vertical: f32,
) -> Option<MouseScrollDelta> {
    if horizontal == 0.0 && vertical == 0.0 {
        None
    } else {
        Some(MouseScrollDelta::LineDelta(-horizontal, vertical))
    }
}

#[cfg(test)]
mod tests {
    use super::android_mouse_scroll_delta;
    use crate::event::MouseScrollDelta;

    #[test]
    fn converts_android_axes_to_winit_content_direction() {
        let cases = [
            ((0.0, 1.0), MouseScrollDelta::LineDelta(0.0, 1.0)),
            ((0.0, -1.0), MouseScrollDelta::LineDelta(0.0, -1.0)),
            ((1.0, 0.0), MouseScrollDelta::LineDelta(-1.0, 0.0)),
            ((-1.0, 0.0), MouseScrollDelta::LineDelta(1.0, 0.0)),
            ((0.5, -2.0), MouseScrollDelta::LineDelta(-0.5, -2.0)),
        ];

        for ((horizontal, vertical), expected) in cases {
            assert_eq!(android_mouse_scroll_delta(horizontal, vertical), Some(expected));
        }
    }

    #[test]
    fn ignores_zero_scroll_axes() {
        assert_eq!(android_mouse_scroll_delta(0.0, 0.0), None);
    }
}
