## Camera
### Camera orbiting


### Camera focusing
1. Wait for LMB click.
2. Shoot ray from camera into scene (use screen space to world space transform).
3. Loop over all `Focusable`:
    1. Store closestes (to camera) sphere intersection point.
4. Attach camera to focused object with closest intersection.
