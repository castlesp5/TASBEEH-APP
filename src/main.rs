pub struct Counter {
    pub count: i32
}

fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut counter = Counter{count: 0};
    loop {
        terminal.draw(|frame| renderer(frame, &counter))?;
        let event = crossterm::event::read()?;
        if let crossterm::event::Event::Key(eve_key) = event {
            match eve_key.code {
                crossterm::event::KeyCode::Char(' ') => {
                    counter.count += 1; 
                }
                crossterm::event::KeyCode::Char('r') => {
                    counter.count = 0; 
                }
                crossterm::event::KeyCode::Char('q') => {break;}
                _ => {}
            }
        }
    }
    Ok(())
}

fn renderer(frame: &mut ratatui::Frame, counter: &Counter) {
    let areas = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Length(1),
        ratatui::layout::Constraint::Length(5),
        ratatui::layout::Constraint::Length(1)
    ]).split(frame.area());
    let header = ratatui::widgets::Paragraph::new("TASBEEH APP V1")
        .alignment(ratatui::layout::Alignment::Center)
        .style(ratatui::style::Style::default()
        .fg(ratatui::style::Color::Black)
        .bg(ratatui::style::Color::Green));
    let paragraph = ratatui::widgets::Paragraph::new(counter.count.to_string())
        .alignment(ratatui::layout::Alignment::Center)
        .block(ratatui::widgets::Block::new()); 
    let footer = ratatui::widgets::Paragraph::new(" [SPACE] = increment       <R> = Reset       <Q> = exit ")
        .alignment(ratatui::layout::Alignment::Center)
        .style(ratatui::style::Style::default()
        .fg(ratatui::style::Color::Black)
        .bg(ratatui::style::Color::Green));
    frame.render_widget(header, areas[0]);
    frame.render_widget(paragraph, areas[1]);
    frame.render_widget(footer, areas[2]);
}
