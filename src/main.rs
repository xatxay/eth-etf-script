pub mod bybit_order;
pub mod bybit_struct;
pub mod bybit_ws;

use std::{env, io};

use bybit_order::place_bybit_order;
use bybit_ws::bybit_ws;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("api key is missing");
    let api_secret = env::var("API_SECRET").expect("api secret is missing");
    let recv_window = "10000";
    let place_order_url = env::var("PLACE_ORDER_URL").expect("api url is missing");
    let ticker = "ETHUSDT";
    let quantity = "5";
    let price = 2000.00; //testing

    let mut space_count = 0;
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    tokio::spawn(async move {
        bybit_ws(&ticker).await;
    });

    loop {
        println!("\rsadsadasd");
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(' ') => {
                    space_count += 1;
                    if space_count == 3 {
                        println!("\renter trade");
                        place_bybit_order(
                            &api_key,
                            &api_secret,
                            recv_window,
                            &place_order_url,
                            ticker,
                            &price,
                            &quantity,
                        )
                        .await
                        .expect("\rError placing order");
                        space_count = 0;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
