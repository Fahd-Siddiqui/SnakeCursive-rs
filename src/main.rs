mod game;
mod score_tracker;

use cursive::{direction::Direction, event::{Event, EventResult}, theme::{BaseColor, Color, ColorStyle}, view::CannotFocus, views::{Button, Dialog, LinearLayout, Panel, SelectView}, Cursive, Printer, Vec2, views};
use cursive::event::Key;
use cursive::view::{IntoBoxedView, Nameable};
use crate::game::{GameResult, MovementDirection, SnakeGame};
use crate::score_tracker::ScoreTracker;


fn main() {
    let mut cursive_runnable = cursive::default();
    let score_tracker: ScoreTracker = ScoreTracker::new();

    cursive_runnable.add_layer(
        Dialog::new()
            .title("Snake")
            .padding_lrtb(2, 2, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("  New game   ", move |s| show_options(s, score_tracker)))
                    .child(Button::new_raw(" Best scores ", move |s| show_best_scores(s, score_tracker)))
                    .child(Button::new_raw("    Exit     ", |s| s.quit())),
            ),
    );


    cursive_runnable.run();
}

fn show_options(cursive_runnable: &mut Cursive, score_tracker: ScoreTracker) {
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
                    .on_submit(move |s, option| {
                        s.pop_layer();
                        new_game(s, *option, score_tracker);
                    }),
            )
            .dismiss_button("Back"),
    );
}

fn new_game(cursive_runnable: &mut Cursive, options: game::Options, score_tracker: ScoreTracker) {
    let mut linear_layout = LinearLayout::vertical();
    let score_board = views::TextView::new_with_content(views::TextContent::new("Score: "));
    let game_board = Panel::new(SnakeGame::new(options, score_tracker));
    linear_layout.add_child(score_board.with_name("score"));
    linear_layout.add_child(game_board);

    cursive_runnable.add_layer(
        Dialog::new()
            .title("Snake")
            .content(linear_layout.into_boxed_view())
            .button("Quit", |s| {
                s.pop_layer();
            }),
    );
    cursive_runnable.set_fps(4);
}

fn show_best_scores(cursive_runnable: &mut Cursive, score_tracker: ScoreTracker) {
    cursive_runnable.add_layer(Dialog::info(format!("Not yet! {}", score_tracker.get_last_score())).title("Scores"));
}

impl cursive::view::View for SnakeGame {
    fn draw(&self, printer: &Printer) {
        for (index, element) in self.cells.iter().enumerate() {
            let x = index % self.size.x;
            let y = index / self.size.x;

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::Dark(BaseColor::White)),
                |printer| printer.print((x, y), element.get_string_representation()),
            );
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.size.map_x(|x| x)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if self.is_paused && event != Event::Char('p') {
            return EventResult::Ignored;
        }

        let game_result = match event {
            Event::Char('p') => {
                if !self.is_paused {
                    self.is_paused = true;
                    GameResult::Continue
                } else {
                    self.is_paused = false;
                    GameResult::Continue
                }
            }

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

        let score = *self.get_last_score();
        let direction = self.get_direction().clone();
        // let formatted_best_scores = self.update_and_get_best_scores().clone();

        if game_result == GameResult::WallCollision || game_result == GameResult::SnakeCollision {
            return EventResult::with_cb(move |s| {
                s.add_layer(Dialog::text(format!("Game Over, Score: {}", score)).button("Ok", |s| {
                    s.pop_layer();
                    s.pop_layer();
                }));
            });
        }

        EventResult::with_cb(move |s| {
            if [MovementDirection::South, MovementDirection::North].contains(&direction) {
                s.set_fps(3);
            } else {
                s.set_fps(4);
            }

            s.call_on_name(
                "score",
                |view: &mut views::TextView| {
                    view.set_content(format!("Score: {}", score));
                },
            );
        })
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}
