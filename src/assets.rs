use crate::state::ChessState;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Component, Debug, Default)]
pub struct BoardAssets {
    pub dark_square_bytes: Vec<u8>,
    pub light_square_bytes: Vec<u8>,
    pub square_selected_bytes: Vec<u8>,
    pub legal_move_square_bytes: Vec<u8>,
    pub dark_square_handle: Handle<Image>,
    pub light_square_handle: Handle<Image>,
    pub square_selected_handle: Handle<Image>,
    pub legal_move_square_handle: Handle<Image>,
    pub bp: Handle<Image>,
    pub br: Handle<Image>,
    pub bn: Handle<Image>,
    pub bb: Handle<Image>,
    pub bq: Handle<Image>,
    pub bk: Handle<Image>,
    pub wp: Handle<Image>,
    pub wr: Handle<Image>,
    pub wn: Handle<Image>,
    pub wb: Handle<Image>,
    pub wq: Handle<Image>,
    pub wk: Handle<Image>,
}

#[derive(Component, Debug, Clone, Default)]
pub struct TextAssets {
    pub regular_font_handle: Handle<Font>,
    pub bold_font_handle: Handle<Font>,
    pub emoji_font_handle: Handle<Font>,
}

pub fn load_assets(
    mut state: ResMut<State<ChessState>>,
    mut board_assets: ResMut<BoardAssets>,
    mut fen_assets: ResMut<TextAssets>,
    mut textures: ResMut<Assets<Image>>,
    mut fonts: ResMut<Assets<Font>>,
) {
    let dark_square_bytes = include_bytes!("../assets/board/dark-square.data");
    let light_square_bytes = include_bytes!("../assets/board/light-square.data");
    let square_selected_bytes = include_bytes!("../assets/board/square-selected.data");
    let legal_move_square_bytes = include_bytes!("../assets/board/legal-move-square.data");
    let bq_bytes = include_bytes!("../assets/pieces/bq.data");
    let bk_bytes = include_bytes!("../assets/pieces/bk.data");
    let br_bytes = include_bytes!("../assets/pieces/br.data");
    let bn_bytes = include_bytes!("../assets/pieces/bn.data");
    let bb_bytes = include_bytes!("../assets/pieces/bb.data");
    let bp_bytes = include_bytes!("../assets/pieces/bp.data");

    let wq_bytes = include_bytes!("../assets/pieces/wq.data");
    let wk_bytes = include_bytes!("../assets/pieces/wk.data");
    let wr_bytes = include_bytes!("../assets/pieces/wr.data");
    let wn_bytes = include_bytes!("../assets/pieces/wn.data");
    let wb_bytes = include_bytes!("../assets/pieces/wb.data");
    let wp_bytes = include_bytes!("../assets/pieces/wp.data");

    let to_image = |b: &[u8; 10000]| {
        bevy::prelude::Image::new(
            Extent3d {
                width: 50,
                height: 50,
                ..default()
            },
            TextureDimension::D2,
            b.to_vec(),
            TextureFormat::Rgba8Unorm,
        )
    };

    let dark_square = to_image(dark_square_bytes);
    let square_selected = to_image(square_selected_bytes);
    let light_square = to_image(light_square_bytes);
    let legal_move_square = to_image(legal_move_square_bytes);

    let bq = to_image(bq_bytes);
    let bk = to_image(bk_bytes);
    let br = to_image(br_bytes);
    let bn = to_image(bn_bytes);
    let bb = to_image(bb_bytes);
    let bp = to_image(bp_bytes);

    let wq = to_image(wq_bytes);
    let wk = to_image(wk_bytes);
    let wr = to_image(wr_bytes);
    let wn = to_image(wn_bytes);
    let wb = to_image(wb_bytes);
    let wp = to_image(wp_bytes);

    board_assets.dark_square_handle = textures.add(dark_square);
    board_assets.light_square_handle = textures.add(light_square);
    board_assets.square_selected_handle = textures.add(square_selected);
    board_assets.legal_move_square_handle = textures.add(legal_move_square);

    board_assets.bq = textures.add(bq);
    board_assets.bk = textures.add(bk);
    board_assets.br = textures.add(br);
    board_assets.bn = textures.add(bn);
    board_assets.bb = textures.add(bb);
    board_assets.bp = textures.add(bp);

    board_assets.wq = textures.add(wq);
    board_assets.wk = textures.add(wk);
    board_assets.wr = textures.add(wr);
    board_assets.wn = textures.add(wn);
    board_assets.wb = textures.add(wb);
    board_assets.wp = textures.add(wp);

    let regular_font_bytes = include_bytes!("../assets/font/NotoSansMono/NotoSansMono-Regular.ttf");
    let bold_font_bytes = include_bytes!("../assets/font/NotoSansMono/NotoSansMono-Bold.ttf");
    let emoji_font_bytes = include_bytes!("../assets/font/NotoEmoji/NotoEmoji-Bold.ttf");

    let regular_font = Font::try_from_bytes(regular_font_bytes.to_vec());
    let bold_font = Font::try_from_bytes(bold_font_bytes.to_vec());
    let emoji_font = Font::try_from_bytes(emoji_font_bytes.to_vec());
    fen_assets.regular_font_handle = fonts.add(regular_font.unwrap());
    fen_assets.bold_font_handle = fonts.add(bold_font.unwrap());
    fen_assets.emoji_font_handle = fonts.add(emoji_font.unwrap());

    state.set(ChessState::Loaded).unwrap();
}
