mod game;

use cursive::{direction::Direction, event::{Event, EventResult}, theme::{BaseColor, Color, ColorStyle}, view::CannotFocus, views::{Button, Dialog, LinearLayout, Panel, SelectView}, Cursive, Printer, Vec2, CursiveExt};
use cursive::event::Key;
use crate::game::{GameResult, MovementDirection};

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
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

    siv.run();
}

fn show_options(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Select difficulty")
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

struct BoardView {
    // Actual board, unknown to the player.
    board: game::Board,
}

impl BoardView {
    pub fn new(options: game::Options) -> Self {
        let board = game::Board::new(options);
        BoardView {
            board,
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for (index, element) in self.board.cells.iter().enumerate() {
            let x = index % self.board.size.x;
            let y = index / self.board.size.x;

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::Dark(BaseColor::White)),
                |printer| printer.print((x, y), element.to_string()),
            );
        }
    }


    fn on_event(&mut self, event: Event) -> EventResult {
        let game_result = match event {
            // pause/resume the stopwatch when pressing "Space"
            Event::Char(' ') => {
                GameResult::Continue
            }

            Event::Key(Key::Up) => {
                self.board.move_forward(MovementDirection::North)
            }

            Event::Key(Key::Down) => {
                self.board.move_forward(MovementDirection::South)
            }

            Event::Key(Key::Left) => {
                self.board.move_forward(MovementDirection::West)
            }

            Event::Key(Key::Right) => {
                self.board.move_forward(MovementDirection::East)
            }

            _ => {
                self.board.move_forward(MovementDirection::None)
            },
        };

        if game_result == GameResult::WallCollision || game_result == GameResult::SnakeCollision{
            return EventResult::with_cb(|s| {
                s.add_layer(Dialog::text("Game Over").button("Ok", |s| {
                    s.pop_layer();
                    // s.pop_layer();
                }));
            })
        }

        self.board.move_forward(MovementDirection::None);
        return EventResult::Consumed(None);
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.board.size.map_x(|x| x)
    }
}

fn new_game(siv: &mut Cursive, options: game::Options) {
    let board_view = BoardView::new(options);

    let text = format!("Quit");

    siv.add_layer(
        Dialog::new()
            .title("Snake")
            .content(LinearLayout::horizontal().child(Panel::new(board_view)))
            .button(text, |s| {
                s.pop_layer();
            }),
    );
    siv.set_fps(1);
    // siv.set_autorefresh(true);
    siv.run();


//     siv.add_layer(Dialog::info(
//         "Controls:
// Reveal cell:                  left click
// Mark as mine:                 right-click
// Reveal nearby unmarked cells: middle-click",
//     ));
}