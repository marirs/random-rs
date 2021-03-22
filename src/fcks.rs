/*
Some Quotes - Lighter side of life!
Fortune Cookies
*/

use rand::seq::SliceRandom;

static COOKIES: &[&str] = &[
    "It is the invisible force that awakens your higher consciousness",
    "Trust that you're being guided",
    "You're Attracting Everything That Matches Your Energetic Vibration",
    "The important key to remember is to be aware of your thoughts and feelings at all times in order to attract and ",
    "manifest what you really desire in your life ",
    "Once you begin to understand and truly master your thoughts and feelings, that's when you see how you create ",
    "your own reality",
    "You are constantly changing as you walk the spiral path – and most importantly, your answer to \"who am I?\" is ",
    "changing, too ",
    "There is nothing stagnant in the Universe and everything is always in motion moving forward",
    "When you work hard for something you love, you feel passion",
    "You are the individual flame from the One Source Creator",
    "Your past was a training ground where the Universe prepared and equipped you for your next step",
    "When you realize your old path was filled with lessons that helped you grow roots, you will then begin to see ",
    "your new path filled with opportunities for expansion",
    "As you are continuously growing, you are becoming more true to yourself",
    "Remember, the bonds you forge with the people around you directly shape the quality of your existence",
    "As the leader of your own life, you can control your thinking"
];

pub fn fortune_cookie() -> &'static str {
    let r = COOKIES
        .choose(&mut rand::thread_rng())
        .unwrap_or(&COOKIES[0]);
    r
}
