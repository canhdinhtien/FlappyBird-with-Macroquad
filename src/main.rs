/*
    Hello, World
    CanhDinhTien
*/

use macroquad::{audio::*, prelude::*};

const GRAVITY: f32 = 0.5;
const JUMP_STRENGTH: f32 = -8.5;
const PIPE_WIDTH: f32 = 50.0;
const PIPE_GAP: f32 = 150.0;

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut state = GameState::MainMenu;
    let mut score = 0;
    let mut high_scores = 0;

    let mut bird_y = screen_height() / 2.0;
    let mut bird_velocity = 0.0;
    let mut pipes = vec![(screen_width(), random_pipe_height())];

    let background_texture: Texture2D = load_texture("background-day.png").await.unwrap();
    let background_texture2: Texture2D = load_texture("background-night.png").await.unwrap();
    let bird_texture = load_texture("bird.png").await.unwrap();

    let fly_sound = load_sound("fly.wav").await.unwrap();
    let crash_sound = load_sound("crash.wav").await.unwrap();
    let score_sound = load_sound("score.wav").await.unwrap();
    // let background_sound = load_sound("background.wav").await.unwrap();

    // play_sound(
    //     background_sound,
    //     PlaySoundParams {
    //         looped: true, 
    //         volume: 0.5,  
    //     },
    // );

    loop {
        clear_background(SKYBLUE);

        match state {
            GameState::MainMenu => {

                draw_text("Flappy Bird", screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 100.0, 50.0, WHITE);
                if button("Start", screen_width() / 2.0 - 100.0, screen_height() / 2.0) {

                    score = 0;
                    bird_y = screen_height() / 2.0;
                    bird_velocity = 0.0;
                    pipes.clear();
                    pipes.push((screen_width(), random_pipe_height()));

                    state = GameState::Playing;
                }

                if button("Exit", screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 50.0) {
                    break;
                }

                if is_key_pressed(KeyCode::Enter){
                    score = 0;
                    bird_y = screen_height() / 2.0;
                    bird_velocity = 0.0;
                    pipes.clear();
                    pipes.push((screen_width(), random_pipe_height()));

                    state = GameState::Playing;
                }
                else if is_key_pressed(KeyCode::Escape){
                    break;
                }
            }

            GameState::Playing => {
                bird_velocity += GRAVITY;
                bird_y += bird_velocity;

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up){
                    play_sound_once(fly_sound);
                    bird_velocity = JUMP_STRENGTH;
                }

                for (x, _) in &mut pipes {
                    *x -= 2.0 * (score+6) as f32 / 6.0;
                }

                if pipes[0].0 < 0.0 {
                    pipes.remove(0);
                    pipes.push((screen_width(), random_pipe_height()));
                    score += 1;
                    play_sound_once(score_sound);
                }
                

                if bird_y < 0.0 || bird_y > screen_height() || pipes.iter().any(|(pipe_x, pipe_y)| bird_collides_with_pipe(bird_y, *pipe_x, *pipe_y)) {
                    play_sound_once(crash_sound);
                    state = GameState::GameOver;
                }

                if (score/3)%2 == 0{ 
                    draw_texture_ex(
                        background_texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width(), screen_height())),
                            ..Default::default()
                        },
                    );
                }
                else {
                    draw_texture_ex(
                        background_texture2,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(screen_width(), screen_height())),
                            ..Default::default()
                        },
                    );
                }

                draw_texture(bird_texture, 100.0 - bird_texture.width() / 2.0, bird_y - bird_texture.height() / 2.0, WHITE);

                for (pipe_x, pipe_y) in &pipes {
                    draw_rectangle(*pipe_x, 0.0, PIPE_WIDTH, *pipe_y, BLUE);
                    draw_rectangle(*pipe_x, *pipe_y + PIPE_GAP, PIPE_WIDTH, screen_height(), BLUE);
                }

                draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE);
                draw_text(&format!("High Score: {}", high_scores), 10.0, 40.0, 30.0, WHITE);
            }
            GameState::GameOver => {
                if score > high_scores{
                    high_scores = score;
                }

                draw_text("Game Over!", screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 100.0, 50.0, RED);
                draw_text(&format!("Score: {}", score), screen_width() / 2.5 - 100.0, screen_height() / 2.0, 30.0, WHITE);
                draw_text(&format!("High Score: {}", high_scores), screen_width() / 1.5 - 100.0, screen_height() / 2.0, 30.0, WHITE);
                if button("Play again", screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 50.0) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space){
                    score = 0;
                    bird_y = screen_height() / 2.0;
                    bird_velocity = 0.0;
                    pipes.clear();
                    pipes.push((screen_width(), random_pipe_height()));

                    state = GameState::Playing;
                }

                if button("Exit", screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 100.0) || is_key_pressed(KeyCode::Escape){
                    break;
                }
            }
        }

        next_frame().await;
    }
}

fn random_pipe_height() -> f32 {
    rand::gen_range(25.0, screen_height() - 200.0)
}

fn bird_collides_with_pipe(bird_y: f32, pipe_x: f32, pipe_y: f32) -> bool {
    let bird_x = 100.0;
    let bird_radius = 20.0;

    let pipe_top = pipe_y;
    let pipe_bottom = pipe_y + PIPE_GAP;

    let bird_in_pipe_x = bird_x + bird_radius > pipe_x && bird_x - bird_radius < pipe_x + PIPE_WIDTH;
    let bird_in_pipe_y = bird_y - bird_radius < pipe_top || bird_y + bird_radius > pipe_bottom;

    bird_in_pipe_x && bird_in_pipe_y
}

fn button(text: &str, x: f32, y: f32) -> bool {
    let mouse_pos = mouse_position();
    let button_rect = Rect::new(x, y, 200.0, 40.0);

    let is_hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
    draw_rectangle(x, y, 200.0, 40.0, if is_hovered { GRAY } else { DARKGRAY });
    draw_text(text, x + 10.0, y + 25.0, 30.0, WHITE);

    is_hovered && is_mouse_button_pressed(MouseButton::Left)
}
