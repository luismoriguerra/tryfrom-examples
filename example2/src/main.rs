
#[derive(Debug, PartialEq)]
enum GameMode {
    SinglePlayer,
    MultiPlayer,
    Cooperative,
}

impl TryFrom<&str> for GameMode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_ref() {
            "singleplayer" => Ok(GameMode::SinglePlayer),
            "multiplayer" => Ok(GameMode::MultiPlayer),
            "cooperative" => Ok(GameMode::Cooperative),
            _ => Err("Invalid game mode".to_string()),
        }
    }
}


fn main() {
    let maybe_mode = GameMode::try_from("singleplayer");

    match maybe_mode {
        Ok(mode) => println!("Game mode is: {:?}", mode),
        Err(err) => println!("Error: {}", err),
    }

    let mode = GameMode::try_from("singleplayer");
    assert_eq!(mode, Ok(GameMode::SinglePlayer));

   
}
