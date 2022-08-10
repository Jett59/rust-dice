use rand::Rng;
use tts::Tts;

fn say(text: &str, tts: &mut Tts) {
    tts.speak(text, true).expect("Failed to speak");
}

fn main() {
    let mut tts = Tts::default().expect("Failed to make the tts");
        say("Welcome to the talking dice app. Press enter to receive a random number.", &mut tts);
        loop {
            let mut rng = rand::thread_rng();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let number = rng.gen_range(1..=6);
            println!("{}", number);
            say(&format!("{}", number), &mut tts);
        }
}
