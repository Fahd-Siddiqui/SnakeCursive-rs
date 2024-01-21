mod game;

use cursive::{direction::Direction, event::{Event, EventResult}, theme::{BaseColor, Color, ColorStyle}, view::CannotFocus, views::{Button, Dialog, LinearLayout, Panel, SelectView}, Cursive, Printer, Vec2, CursiveExt};
use cursive::event::Key;
use cursive::view::IntoBoxedView;
use crate::game::{GameResult, MovementDirection, SnakeGame};


fn main() {
    let mut cursive_runnable = cursive::default();

    cursive_runnable.add_layer(
        Dialog::new()
            .title("Snake")
            .padding_lrtb(2, 2, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("  New game   ", show_options))
                    .child(Button::new_raw(" Best scores ", |s| {
                        s.add_layer(Dialog::info("Not yet!").title("Scores"))
                    }))
                    .child(Button::new_raw("    Exit     ", |s| s.quit())),
            ),
    );

    cursive_runnable.run();
}

fn show_options(cursive_runnable: &mut Cursive) {
    cursive_runnable.add_layer(
        Dialog::new()
            .title("Select size")
            .content(
                SelectView::new()
                    .item(
                        "Small",
                        game::Options {
                            size: Vec2::new(16 * 4, 16)
                        },
                    )
                    .item(
                        "Medium",
                        game::Options {
                            size: Vec2::new(32 * 4, 32),

                        },
                    )
                    .item(
                        "Large",
                        game::Options {
                            size: Vec2::new(64 * 4, 64),
                        },
                    )
                    .on_submit(|s, option| {
                        s.pop_layer();
                        new_game(s, *option);
                    }),
            )
            .dismiss_button("Back"),
    );
}

fn new_game(cursive_runnable: &mut Cursive, options: game::Options) {
    let mut linear_layout = LinearLayout::vertical();
    // TODO implement score
    let score_board = cursive::views::TextView::new_with_content(cursive::views::TextContent::new("Score: "));
    let game_board = Panel::new(SnakeGame::new(options));
    linear_layout.add_child(score_board);
    linear_layout.add_child(game_board);

    cursive_runnable.add_layer(
        Dialog::new()
            .title("Snake")
            .content(linear_layout.into_boxed_view())
            .button("Quit", |s| {
                s.pop_layer();
            }),
    );
    cursive_runnable.set_fps(5);
    cursive_runnable.run();
}

impl cursive::view::View for SnakeGame {
    fn draw(&self, printer: &Printer) {
        for (index, element) in self.cells.iter().enumerate() {
            let x = index % self.size.x;
            let y = index / self.size.x;

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::Dark(BaseColor::White)),
                |printer| printer.print((x, y), element.to_string()),
            );
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size.map_x(|x| x)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let game_result = match event {
            Event::Key(Key::Up) => {
                self.move_forward(MovementDirection::North)
            }

            Event::Key(Key::Down) => {
                self.move_forward(MovementDirection::South)
            }

            Event::Key(Key::Left) => {
                self.move_forward(MovementDirection::West)
            }

            Event::Key(Key::Right) => {
                self.move_forward(MovementDirection::East)
            }

            _ => {
                self.move_forward(MovementDirection::None)
            }
        };

        let score = self.score.clone();

        if game_result == GameResult::WallCollision || game_result == GameResult::SnakeCollision {
            return EventResult::with_cb(move |s| {
                s.add_layer(Dialog::text(format!("Game Over, Score: {}", score)).button("Ok", |s| {
                    s.pop_layer();
                    s.pop_layer();
                }));
            });
        }

        return EventResult::Consumed(None);

    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}
