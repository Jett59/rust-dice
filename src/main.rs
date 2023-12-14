use std::{error::Error, io::{stdout, Write}};

use rand::Rng;
use tts::Tts;

fn say(text: &str, tts: &mut Tts) -> Result<(), Box<dyn Error>> {
    tts.speak(text, true)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tts = Tts::default()?;
    let voices = tts
        .voices()?
        .into_iter()
        .filter(|voice| {
            // I have observed that some of the voices give errors when you try to use them, so we have to filter them out.
            tts.set_voice(voice).is_ok() && tts.speak(" ", true).is_ok()
        })
        .collect::<Vec<_>>();
    for (i, voice) in voices.iter().enumerate() {
        println!("{}: {}", i + 1, voice.name());
    }
    {
        say("Choose a voice", &mut tts)?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let index = input.trim().parse::<usize>()? - 1;
        tts.set_voice(&voices[index])?;
    }
    say(
        "Welcome to the talking dice app. Press enter to receive a random number.",
        &mut tts,
    )?;
    let mut rng = rand::thread_rng();
    loop {
        let number = rng.gen_range(1..=6);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        say(&format!("{}", number), &mut tts)?;
        print!("{}", number);
        stdout().flush()?;
    }
}
