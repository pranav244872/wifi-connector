use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use std::io;
use std::process::Command;

//fn list_networks() -> Result<(), String> {
//    let output = Command::new("nmcli")
//        .arg("device")
//        .arg("wifi")
//        .arg("list")
//        .output();
//    match output {
//        Ok(output) => {
//            if output.status.success() {
//                let wifi_list = String::from_utf8_lossy(&output.stdout);
//                println!("Wifi List: \n {}", wifi_list);
//                Ok(())
//            } else {
//                // If the command fails (non-zero exit code), print stderr
//                let error_message = String::from_utf8_lossy(&output.stderr);
//                return Err(format!("Failed to list Wifi Networks: {}", error_message));
//            }
//        }
//
//        Err(e) => {
//            // If there's an error running the command, print the error
//            Err(format!("Error running nmcli: {}", e.to_string()))
//        }
//    }
//}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (Press q to quit)").white();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}
