use iced_todo::Todos;

fn main() -> iced::Result {
    iced::application(Todos::new, Todos::update, Todos::view)
        .title("Todos - React Hooks")
        .window_size((500.0, 800.0))
        .run()
}
