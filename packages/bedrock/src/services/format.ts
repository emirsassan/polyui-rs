export function getPolyFormat(codec: Codec): ModelFormat {
    return new ModelFormat({
        id: 'polyui',
        name: 'PolyUI Format',
        icon: 'fa-brackets-curly',
        rotate_cubes: true,
        box_uv: true,
        optional_box_uv: false,
        single_texture: true,
        bone_rig: true,
        centered_grid: true,
        animated_textures: true,
        animation_files: true,
        animation_mode: true,
        integer_size: true,
        locators: true,
        codec: codec,
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onActivation: () => {}
    });
}