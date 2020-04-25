use iced::{
    executor, pane_grid::Axis, pane_grid, scrollable, Align, Application, Command, Container,
    Element, Length, PaneGrid, Scrollable, Settings, Text,
};

use risc::computer;

pub fn main() {
    Gui::run(Settings::default())
}

struct Gui {
    panes_grid_state: pane_grid::State<Content>,
    computer: computer::Computer
}

enum PaneType {
    Registers,
    Memory
}

type Message = ();

impl Application for Gui {
    type Executor = executor::Null;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Gui, Command<Self::Message>) {
        let (mut panes_grid_state, pane1) = pane_grid::State::new(Content::new(PaneType::Registers));
        panes_grid_state.split(Axis::Vertical, &pane1, Content::new(PaneType::Memory));

        (
            Gui {
                panes_grid_state: panes_grid_state,
                computer: computer::Computer::new()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Wirth RISC Computer GUI")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let computer = &mut self.computer;

        let pane_grid = PaneGrid::new(&mut self.panes_grid_state, |pane, content, _focus| {
            content.view(pane, computer)
        })
        .width(Length::Fill)
        .height(Length::Fill);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

struct Content {
    pane_type: PaneType,
    scroll: scrollable::State,
}

impl Content {
    fn new(pane_type: PaneType) -> Self {
        Content {
            pane_type: pane_type,
            scroll: scrollable::State::new(),
        }
    }
    fn view(&mut self, _pane: pane_grid::Pane, computer: &computer::Computer) -> Element<Message> {

        match self.pane_type {
            PaneType::Registers => {
                self.registers_content(computer)
            }

            PaneType::Memory => {
                self.memory_content()
            }
        }

    }

    fn registers_content(&mut self, computer: &computer::Computer) -> Element<Message> {
        let mut content = Scrollable::new(&mut self.scroll)
        .width(Length::Fill)
        .spacing(10)
        .align_items(Align::Center)
        .push(Text::new("Registers").size(30));

        for (index, reg) in computer.regs.iter().enumerate() {
            content = content.push(Text::new(format!("REG {:02}: 0x{:04X} 0b{:32b}", index, reg, reg)));
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            // .center_y()
            .into()
    }

    fn memory_content(&mut self) -> Element<Message> {
        let content = Scrollable::new(&mut self.scroll)
        .width(Length::Fill)
        .spacing(10)
        .align_items(Align::Center)
        .push(Text::new("Memory").size(30));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
    }
}