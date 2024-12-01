use rand::Rng;
use std::io;
use crossterm::style::Stylize;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};

#[derive(Debug,PartialEq,Clone)]
enum Card {YELLOW(YellowCard),GREEN(GreenCard),RED(RedCard),BLUE(BlueCard),Wild,WildP4,}

#[derive(Debug,PartialEq,Clone)]
enum YellowCard {YZero, YOne, YTwo, YThree, YFour, YFive, YSix, YSeven, YEight, YNine,YSkip, YReverse, YP2,}

#[derive(Debug,PartialEq,Clone)]
enum GreenCard {GZero, GOne, GTwo, GThree, GFour, GFive, GSix, GSeven, GEight, GNine,GSkip, GReverse, GP2,}

#[derive(Debug,PartialEq,Clone)]
enum RedCard {RZero, ROne, RTwo, RThree, RFour, RFive, RSix, RSeven, REight, RNine,RSkip, RReverse, RP2,}

#[derive(Debug,PartialEq,Clone)]
enum BlueCard {BZero, BOne, BTwo, BThree, BFour, BFive, BSix, BSeven, BEight, BNine,BSkip, BReverse, BP2,}

fn all_yellow_cards() -> Vec<Card> //This function gives all yellow cards
{
    vec![  Card::YELLOW(YellowCard::YZero),Card::YELLOW(YellowCard::YOne),Card::YELLOW(YellowCard::YTwo),Card::YELLOW(YellowCard::YThree),
        Card::YELLOW(YellowCard::YFour),Card::YELLOW(YellowCard::YFive),Card::YELLOW(YellowCard::YSix),Card::YELLOW(YellowCard::YSeven),
        Card::YELLOW(YellowCard::YEight),Card::YELLOW(YellowCard::YNine),Card::YELLOW(YellowCard::YSkip),Card::YELLOW(YellowCard::YReverse),Card::YELLOW(YellowCard::YP2), ]
    }

fn all_green_cards() -> Vec<Card> //This function gives all green cards
{
    vec![ Card::GREEN(GreenCard::GZero),Card::GREEN(GreenCard::GOne),Card::GREEN(GreenCard::GTwo),Card::GREEN(GreenCard::GThree),
        Card::GREEN(GreenCard::GFour),Card::GREEN(GreenCard::GFive),Card::GREEN(GreenCard::GSix),Card::GREEN(GreenCard::GSeven),
        Card::GREEN(GreenCard::GEight),Card::GREEN(GreenCard::GNine),Card::GREEN(GreenCard::GSkip),Card::GREEN(GreenCard::GReverse),Card::GREEN(GreenCard::GP2),]
}

fn all_red_cards() -> Vec<Card>//This function gives all red cards
 {
    vec![Card::RED(RedCard::RZero),Card::RED(RedCard::ROne),Card::RED(RedCard::RTwo),Card::RED(RedCard::RThree),
        Card::RED(RedCard::RFour),Card::RED(RedCard::RFive),Card::RED(RedCard::RSix),Card::RED(RedCard::RSeven),
        Card::RED(RedCard::REight),Card::RED(RedCard::RNine),Card::RED(RedCard::RSkip),Card::RED(RedCard::RReverse),Card::RED(RedCard::RP2),]
}

fn all_blue_cards() -> Vec<Card> //This function gives all blue cards
{
    vec![
        Card::BLUE(BlueCard::BZero),Card::BLUE(BlueCard::BOne),Card::BLUE(BlueCard::BTwo),Card::BLUE(BlueCard::BThree),
        Card::BLUE(BlueCard::BFour),Card::BLUE(BlueCard::BFive),Card::BLUE(BlueCard::BSix),Card::BLUE(BlueCard::BSeven),
        Card::BLUE(BlueCard::BEight),Card::BLUE(BlueCard::BNine),Card::BLUE(BlueCard::BSkip),Card::BLUE(BlueCard::BReverse),Card::BLUE(BlueCard::BP2),]
}

fn match_card_value(card: &Card) -> i64//This function returns the relative value for each card.
 {
    match card {
        Card::YELLOW(y) => match y {
            YellowCard::YZero => 0,YellowCard::YOne => 1,YellowCard::YTwo => 2,YellowCard::YThree => 3,YellowCard::YFour => 4,YellowCard::YFive => 5,YellowCard::YSix => 6,
            YellowCard::YSeven => 7,YellowCard::YEight => 8,YellowCard::YNine => 9,YellowCard::YSkip | YellowCard::YReverse | YellowCard::YP2 => 20,},
        Card::GREEN(g) => match g {
            GreenCard::GZero => 0,GreenCard::GOne => 1,GreenCard::GTwo => 2,GreenCard::GThree => 3,GreenCard::GFour => 4,GreenCard::GFive => 5,GreenCard::GSix => 6,
            GreenCard::GSeven => 7,GreenCard::GEight => 8,GreenCard::GNine => 9,GreenCard::GSkip | GreenCard::GReverse | GreenCard::GP2 => 20,},
        Card::RED(r) => match r {
            RedCard::RZero => 0,RedCard::ROne => 1,RedCard::RTwo => 2,RedCard::RThree => 3,RedCard::RFour => 4,RedCard::RFive => 5,RedCard::RSix => 6,
            RedCard::RSeven => 7,RedCard::REight => 8,RedCard::RNine => 9,RedCard::RSkip | RedCard::RReverse | RedCard::RP2 => 20,},
        Card::BLUE(b) => match b {
            BlueCard::BZero => 0,BlueCard::BOne => 1,BlueCard::BTwo => 2,BlueCard::BThree => 3,BlueCard::BFour => 4,BlueCard::BFive => 5,BlueCard::BSix => 6,
            BlueCard::BSeven => 7,BlueCard::BEight => 8,BlueCard::BNine => 9,BlueCard::BSkip | BlueCard::BReverse | BlueCard::BP2 => 20,},Card::Wild | Card::WildP4 => 50,
        }
        }

fn card_value(cards: &Vec<Card>)-> i64//This function returns the sum of all card values in given vector
{
    let mut sum: i64 = 0;
    for i in 0..cards.len(){ 
        let card_value: i64 =match_card_value(&cards[i]);
        sum=sum+card_value;}
    sum
}

fn match_card_random() -> Card //This function returns a random card from 52 Uno Cards
{
    let mut rng = rand::thread_rng();
    match rng.gen_range(1..=54) {
        1 => Card::YELLOW(YellowCard::YZero),2 => Card::YELLOW(YellowCard::YOne),3 => Card::YELLOW(YellowCard::YTwo),4 => Card::YELLOW(YellowCard::YThree),5 => Card::YELLOW(YellowCard::YFour),
        6 => Card::YELLOW(YellowCard::YFive),7 => Card::YELLOW(YellowCard::YSix),8 => Card::YELLOW(YellowCard::YSeven),9 => Card::YELLOW(YellowCard::YEight),10 => Card::YELLOW(YellowCard::YNine),
        11 => Card::YELLOW(YellowCard::YSkip),12 => Card::YELLOW(YellowCard::YReverse),13 => Card::YELLOW(YellowCard::YP2),14 => Card::GREEN(GreenCard::GZero),15 => Card::GREEN(GreenCard::GOne),
        16 => Card::GREEN(GreenCard::GTwo),17 => Card::GREEN(GreenCard::GThree),18 => Card::GREEN(GreenCard::GFour),19 => Card::GREEN(GreenCard::GFive),20 => Card::GREEN(GreenCard::GSix),
        21 => Card::GREEN(GreenCard::GSeven),22 => Card::GREEN(GreenCard::GEight),23 => Card::GREEN(GreenCard::GNine),24 => Card::GREEN(GreenCard::GSkip),25 => Card::GREEN(GreenCard::GReverse),
        26 => Card::GREEN(GreenCard::GP2),27 => Card::RED(RedCard::RZero),28 => Card::RED(RedCard::ROne),29 => Card::RED(RedCard::RTwo),30 => Card::RED(RedCard::RThree),
        31 => Card::RED(RedCard::RFour),32 => Card::RED(RedCard::RFive),33 => Card::RED(RedCard::RSix),34 => Card::RED(RedCard::RSeven),35 => Card::RED(RedCard::REight),
        36 => Card::RED(RedCard::RNine),37 => Card::RED(RedCard::RSkip),38 => Card::RED(RedCard::RReverse),39 => Card::RED(RedCard::RP2),40 => Card::BLUE(BlueCard::BZero),
        41 => Card::BLUE(BlueCard::BOne),42 => Card::BLUE(BlueCard::BTwo),43 => Card::BLUE(BlueCard::BThree),44 => Card::BLUE(BlueCard::BFour),45 => Card::BLUE(BlueCard::BFive),
        46 => Card::BLUE(BlueCard::BSix),47 => Card::BLUE(BlueCard::BSeven),48 => Card::BLUE(BlueCard::BEight),49 => Card::BLUE(BlueCard::BNine),50 => Card::BLUE(BlueCard::BSkip),
        51 => Card::BLUE(BlueCard::BReverse),52 => Card::BLUE(BlueCard::BP2),53 => Card::Wild,54 => Card::WildP4,_ => unreachable!(),
    }
}

fn card_random( )->Vec<Card>//This function returns a vector of 5 random cards for each player
{
    let mut random_cards:Vec<Card>=Vec::new();
    for _ in 0..5 {
        let chosen_card:Card=match_card_random(); 
        random_cards.push(chosen_card);}
    random_cards
}

fn show_card_status(count: &mut i64,end_time:&mut i64,pcards: &Vec<Card>,bcards:&Vec<Card>,p_card_numbers: &usize,b_card_numbers: &usize,p_card_value: &i64,b_card_value: &i64,)
//This function shows the status of player cards and bot cards,  and win/lose situation
{
    println!("{}","----------------------------------------------------------------------------------------------------------------------------------------------------------".magenta().bold());
    if *p_card_numbers==0 {
        *end_time-=*count;
        println!("{}","Congradulations!You left no card.\n You won the game.".cyan().bold());
        let game_text_1 = "Game ended at time count ".green().bold();
        println!(" {}'{}'",game_text_1,end_time);
        *count=-1;
    }else if *b_card_numbers==0 {
        *end_time-=*count;
        println!("{}","Sorry!Bot left no card.\n Bot won the game.".magenta().bold());
        let game_text_2 = "Game ended at time count ".green().bold();
        println!(" {}'{}'",game_text_2,end_time);
        *count=-1;
    }else{ if *count > 5 {
        print!("{}","Your cards: |".cyan().bold());
        let mut no=1;
        for x in pcards {
            let colored_x = format!("{}. {:?} |",no, x).cyan();
            print!("{}   ", colored_x);
            no+=1;
        }
        let bot_text = "Bot has".magenta().bold(); 
        let cards_text = "cards".magenta().bold();   
        println!("\n{} {} {}.", bot_text,b_card_numbers,cards_text);
          }else if *count > 0 && *count <= 5 {
        let text_1 = "The game will end after".red();
        let text_2 = "times".red();
        println!("{} {} {}.", text_1,count,text_2);
        print!("{}","Your cards: |".cyan().bold());
        let mut no=1;
        for x in pcards {
            let colored_x = format!("{}. {:?} |",no, x).cyan();
            print!("{}   ", colored_x);
            no+=1;
        }
        let p_text = "Your card's value = ".cyan().bold();
        println!("\n {} {}", p_text,p_card_value);
        let bot_text = "Bot has".magenta().bold(); 
        let cards_text = "cards".magenta().bold();   
        println!("\n{} {} {}.", bot_text,b_card_numbers,cards_text);
        let b_text = "Bot card's value = ".magenta().bold();
        println!("{} {}", b_text,b_card_value);
    } else if *count == 0 {
        println!("{}","Times up!".red());
        print!("{}","Your cards: |".cyan().bold());
        let mut no=1;
        for x in pcards {
            let colored_x = format!("{}. {:?} |",no, x).cyan();
            print!("{}   ", colored_x);
            no+=1;
        }
        let p_text = "Your card's value = ".cyan().bold();
        println!("\n{} {}", p_text,p_card_value);
        print!("{}","Bot cards: |".magenta().bold());
        let mut nu=1;
        for x in bcards {
            let colored_x = format!("{}. {:?} |",nu, x).magenta();
            print!("{}   ", colored_x);
            nu+=1;
        }
        let b_text = "\nBot card's value = ".magenta().bold();
        println!("{} {}", b_text,b_card_value);
        if p_card_value < b_card_value {
            println!("{}","Player won the game.".green().bold());
       }else if p_card_value == b_card_value {
            if p_card_numbers < b_card_numbers {
                println!("{}","Player won the game.".green().bold());
           }else if p_card_numbers > b_card_numbers {
                println!("{}","Bot won the game.".red().bold());
           }else {
                println!("{}","Unfortunately, the game was a draw.".dark_blue().bold());}
       }else {
            println!("{}","Bot won the game.".red().bold());}}
        }  
}

fn available(card: &Card) -> Vec<Card>//This function returns a vector of playable cards to fight back the given card
 {
            match card {
                Card::YELLOW(YellowCard::YZero) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GZero),Card::RED(RedCard::RZero),Card::BLUE(BlueCard::BZero),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YOne) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GOne),Card::RED(RedCard::ROne),Card::BLUE(BlueCard::BOne),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YTwo) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GTwo),Card::RED(RedCard::RTwo),Card::BLUE(BlueCard::BTwo),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YThree) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GThree),Card::RED(RedCard::RThree),Card::BLUE(BlueCard::BThree),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YFour) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GFour),Card::RED(RedCard::RFour),Card::BLUE(BlueCard::BFour),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YFive) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GFive),Card::RED(RedCard::RFive),Card::BLUE(BlueCard::BFive),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YSix) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSix),Card::RED(RedCard::RSix),Card::BLUE(BlueCard::BSix),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YSeven) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSeven),Card::RED(RedCard::RSeven),Card::BLUE(BlueCard::BSeven),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YEight) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GEight),Card::RED(RedCard::REight),Card::BLUE(BlueCard::BEight),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YNine) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GNine),Card::RED(RedCard::RNine),Card::BLUE(BlueCard::BNine),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YSkip) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSkip),Card::RED(RedCard::RSkip),Card::BLUE(BlueCard::BSkip),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YReverse) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GReverse),Card::RED(RedCard::RReverse),Card::BLUE(BlueCard::BReverse),Card::Wild,Card::WildP4,]);cards}
                Card::YELLOW(YellowCard::YP2) => {let mut cards = all_yellow_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GP2),Card::RED(RedCard::RP2),Card::BLUE(BlueCard::BP2),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GZero) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YZero),Card::RED(RedCard::RZero),Card::BLUE(BlueCard::BZero),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GOne) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YOne),Card::RED(RedCard::ROne),Card::BLUE(BlueCard::BOne),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GTwo) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YTwo),Card::RED(RedCard::RTwo),Card::BLUE(BlueCard::BTwo),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GThree) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YThree),Card::RED(RedCard::RThree),Card::BLUE(BlueCard::BThree),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GFour) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YFour),Card::RED(RedCard::RFour),Card::BLUE(BlueCard::BFour),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GFive) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YFive),Card::RED(RedCard::RFive),Card::BLUE(BlueCard::BFive),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GSix) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YSix),Card::RED(RedCard::RSix),Card::BLUE(BlueCard::BSix),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GSeven) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YSeven),Card::RED(RedCard::RSeven),Card::BLUE(BlueCard::BSeven),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GEight) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YEight),Card::RED(RedCard::REight),Card::BLUE(BlueCard::BEight),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GNine) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YNine),Card::RED(RedCard::RNine),Card::BLUE(BlueCard::BNine),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GSkip) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YSkip),Card::RED(RedCard::RSkip),Card::BLUE(BlueCard::BSkip),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GReverse) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YReverse),Card::RED(RedCard::RReverse),Card::BLUE(BlueCard::BReverse),Card::Wild,Card::WildP4,]);cards}
                Card::GREEN(GreenCard::GP2) => {let mut cards = all_green_cards();
                    cards.extend(vec![Card::YELLOW(YellowCard::YP2),Card::RED(RedCard::RP2),Card::BLUE(BlueCard::BP2),Card::Wild,Card::WildP4,]);cards}
                Card::RED(RedCard::RZero) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GZero), Card::YELLOW(YellowCard::YZero), Card::BLUE(BlueCard::BZero), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::ROne) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GOne), Card::YELLOW(YellowCard::YOne), Card::BLUE(BlueCard::BOne), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RTwo) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GTwo), Card::YELLOW(YellowCard::YTwo), Card::BLUE(BlueCard::BTwo), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RThree) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GThree), Card::YELLOW(YellowCard::YThree), Card::BLUE(BlueCard::BThree), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RFour) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GFour), Card::YELLOW(YellowCard::YFour), Card::BLUE(BlueCard::BFour), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RFive) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GFive), Card::YELLOW(YellowCard::YFive), Card::BLUE(BlueCard::BFive), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RSix) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSix), Card::YELLOW(YellowCard::YSix), Card::BLUE(BlueCard::BSix), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RSeven) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSeven), Card::YELLOW(YellowCard::YSeven), Card::BLUE(BlueCard::BSeven), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::REight) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GEight), Card::YELLOW(YellowCard::YEight), Card::BLUE(BlueCard::BEight), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RNine) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GNine), Card::YELLOW(YellowCard::YNine), Card::BLUE(BlueCard::BNine), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RSkip) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSkip), Card::YELLOW(YellowCard::YSkip), Card::BLUE(BlueCard::BSkip), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RReverse) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GReverse), Card::YELLOW(YellowCard::YReverse), Card::BLUE(BlueCard::BReverse), Card::Wild, Card::WildP4]);cards}
                Card::RED(RedCard::RP2) => {let mut cards = all_red_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GP2), Card::YELLOW(YellowCard::YP2), Card::BLUE(BlueCard::BP2), Card::Wild, Card::WildP4]);cards}   
                Card::BLUE(BlueCard::BZero) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GZero), Card::RED(RedCard::RZero), Card::YELLOW(YellowCard::YZero), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BOne) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GOne), Card::RED(RedCard::ROne), Card::YELLOW(YellowCard::YOne), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BTwo) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GTwo), Card::RED(RedCard::RTwo), Card::YELLOW(YellowCard::YTwo), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BThree) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GThree), Card::RED(RedCard::RThree), Card::YELLOW(YellowCard::YThree), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BFour) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GFour), Card::RED(RedCard::RFour), Card::YELLOW(YellowCard::YFour), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BFive) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GFive), Card::RED(RedCard::RFive), Card::YELLOW(YellowCard::YFive), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BSix) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSix), Card::RED(RedCard::RSix), Card::YELLOW(YellowCard::YSix), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BSeven) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSeven), Card::RED(RedCard::RSeven), Card::YELLOW(YellowCard::YSeven), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BEight) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GEight), Card::RED(RedCard::REight), Card::YELLOW(YellowCard::YEight), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BNine) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GNine), Card::RED(RedCard::RNine), Card::YELLOW(YellowCard::YNine), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BSkip) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GSkip), Card::RED(RedCard::RSkip), Card::YELLOW(YellowCard::YSkip), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BReverse) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GReverse), Card::RED(RedCard::RReverse), Card::YELLOW(YellowCard::YReverse), Card::Wild, Card::WildP4]);cards}
                Card::BLUE(BlueCard::BP2) => {let mut cards = all_blue_cards();
                    cards.extend(vec![Card::GREEN(GreenCard::GP2), Card::RED(RedCard::RP2), Card::YELLOW(YellowCard::YP2), Card::Wild, Card::WildP4]);cards}
                _=>unreachable!(),}
        }

fn process_wild_card(ava_cards: &mut Vec<Card>, is_player: bool)//Wild  Cards(Wild,WildP4) need to change color.This function helps for color chabging.
{
    let change: i32 = if is_player {
        println!("{}","As it is Wild Card, which color do you want to change? (Yellow: 1, Green: 2, Red: 3, Blue: 4)".cyan());
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Failed to read.");
        ans.trim().parse().unwrap_or(1)
    } else {
        println!("{}","Bot is choosing a color.".magenta());
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=4)
    };
    let new_cards = match change {
        1 => { println!("{}","Changed to yellow color.".yellow().italic()); all_yellow_cards() }
        2 => { println!("{}","Changed to green color.".green().italic()); all_green_cards() }
        3 => { println!("{}","Changed to red color.".red().italic()); all_red_cards() }
        4 => { println!("{}","Changed to blue color.".blue().italic()); all_blue_cards() }
        _ => { println!("{}","Typo Error.Changed to red color as default.".red().bold()); all_red_cards() }
    };
    ava_cards.clear();
    ava_cards.extend(new_cards);
    ava_cards.extend(vec![Card::Wild, Card::WildP4]);
}

fn player_turn(a_cards: &mut Vec<Card>, tf_card: &mut Card, tp_card: &mut Vec<Card>,tp_card_value: &mut i64, tp_card_numbers: &mut usize,tb_card: &mut Vec<Card>, tb_card_value: &mut i64, tb_card_numbers: &mut usize,t_round: &mut i64) 
//When it is player's turn, player has to  decide which card should be played.This function helps to mange the player's choice whether it can be played or not, and  helps to decide the next turn.
{
    println!("{}","Player Turn:".cyan().bold());
    loop {
        println!("{}","Type the card you chose (For 1st card: Type 1, To draw a card: Type 0)".cyan());
        let mut card = String::new();
        io::stdin().read_line(&mut card).expect("Failed to read.");
        let card: usize = card.trim().parse().expect("Failed to read");
        if card > 0 && card <= tp_card.len() {
            let x = &tp_card[card - 1];
            if a_cards.contains(x) {
                *tf_card = x.clone();
                *tp_card_value-=match_card_value(&x);
                tp_card.remove(card-1);
                *tp_card_numbers-=1;
                if *tp_card_numbers == 1 {
                    println!("{}","You have only 1 card left! Press 1 to call!".red());
                    let mut uno_call = String::new();
                    io::stdin().read_line(&mut uno_call).expect("Failed to read.");
                    let uno_call: i32 = uno_call.trim().parse().unwrap_or(0);
                    if uno_call != 1 {
                        for _ in 0..2 {
                            let drawn_card = match_card_random();
                            tp_card.push(drawn_card.clone());
                            *tp_card_value += match_card_value(&drawn_card);
                            *tp_card_numbers += 1;
                        }
                        println!("{}","You failed to call UNO! Draw 2 cards.".red());
                    } else {
                        println!("{}","UNO called successfully!".green());
                    }
                }
                handle_card_playing(a_cards, tf_card, tp_card, tp_card_value,tp_card_numbers, tb_card, tb_card_value,tb_card_numbers, t_round, true);
                break;
          } else {
                println!("{}","The selected card is  not match to play.Please choose appropriate card or type 0 to draw a card.".red());
                continue;   }
       }else if card==0{
                println!("{}","You drew a card.".green());
                let draw_card = match_card_random();
                if a_cards.contains(&draw_card) {
                    println!("{}","Ohh, you can play with the drawn card. Do you want? (To play: 1)".green());
                    let mut play = String::new();
                    io::stdin().read_line(&mut play).expect("Failed to read.");
                    let play: i32 = play.trim().parse().expect("Failed to read");
                    if play == 1 {
                        *tf_card = draw_card.clone();
                        handle_card_playing(a_cards, tf_card, tp_card, tp_card_value,tp_card_numbers, tb_card, tb_card_value,tb_card_numbers, t_round, true);
                    break;
                  } else {
                        println!("{}","You kept the drawn card.".green());
                        *tp_card_value += match_card_value(&draw_card);
                        *tp_card_numbers += 1;
                        tp_card.push(draw_card);
                        *t_round += 1;
                        break;
                    }
              } else {
                    println!("{}","You can't play the drawn card.".red());
                    *tp_card_value += match_card_value(&draw_card);
                    *tp_card_numbers += 1;
                    tp_card.push(draw_card.clone());
                    *t_round += 1;
                    break;
                }
       }else {
            println!("{}","You typed out of range!".red());
            continue;
        }
    }
}

fn bot_turn(a_cards: &mut Vec<Card>, tf_card: &mut Card, tp_card: &mut Vec<Card>,tp_card_value: &mut i64, tp_card_numbers: &mut usize,tb_card: &mut Vec<Card>, tb_card_value: &mut i64, tb_card_numbers: &mut usize,t_round: &mut i64) 
//When it is bot's turn, computer will randomly choose a random,playable card from the bot cards.This function helps it and also help to decide the next turn.
{
    println!("{}","BOT Turn:".magenta().bold());
    let mut found = false;
    let mut index = 0;
    while index < tb_card.len() {
        let x = &tb_card[index];
        if a_cards.contains(x) {
            println!("{}","Bot played a card".magenta().bold());
            *tb_card_value-=match_card_value(&x);
            found = true;
            *tf_card =x.clone();
            *tb_card_numbers-=1;
            tb_card.remove(index); 
            handle_card_playing(a_cards, tf_card, tb_card, tb_card_value,tb_card_numbers, tp_card, tp_card_value,tp_card_numbers, t_round, false);
            break;
      } else {
            index += 1;
        }
    }
    if !found {
        println!("{}","Bot drew a card.".magenta());
        let draw_card = match_card_random();
        if a_cards.contains(&draw_card) {
            *tf_card = draw_card;
            println!("{}","Bot played the drawn card.".magenta());
            handle_card_playing(a_cards, tf_card, tb_card, tb_card_value,tb_card_numbers, tp_card, tp_card_value,tp_card_numbers, t_round, false);
      } else {
            println!("{}","Bot kept the drawn card".magenta());
            *tb_card_value += match_card_value(&draw_card);
            *tb_card_numbers += 1;
            tb_card.push(draw_card);
            *t_round += 1;
        }
    }
}

fn handle_card_playing(cards: &mut Vec<Card>, card_to_play: &Card, _target_pile: &mut Vec<Card>, _target_pile_value: &mut i64, _target_pile_numbers: &mut usize, _draw_pile: &mut Vec<Card>, _draw_pile_value: &mut i64, _draw_pile_numbers: &mut usize, round_counter: &mut i64, is_player: bool) 
//This function manages the special cards (Reverse,Skip,+2,Wild,Wild +4) to activate their respective features
{
    if matches!(card_to_play, Card::YELLOW(YellowCard::YReverse) | Card::GREEN(GreenCard::GReverse) | Card::RED(RedCard::RReverse) | Card::BLUE(BlueCard::BReverse) | Card::YELLOW(YellowCard::YSkip) | Card::GREEN(GreenCard::GSkip) | Card::RED(RedCard::RSkip) | Card::BLUE(BlueCard::BSkip)) {
        *round_counter += 2;
        println!("{}","Turn changes because card is reverse or skip.".green());
  } else if matches!(card_to_play, Card::YELLOW(YellowCard::YP2) | Card::GREEN(GreenCard::GP2) | Card::RED(RedCard::RP2) | Card::BLUE(BlueCard::BP2)) {
        if is_player{
            for _ in 0..2 {
                let plus_card = match_card_random();
                *_draw_pile_value += match_card_value(&plus_card);
                *_draw_pile_numbers += 1;
                _draw_pile.push(plus_card);
            }
            println!("{}","+2 card played,bot drew two cards".magenta());
        }
        else{
            for _ in 0..2 {
                let plus_card = match_card_random();
                *_draw_pile_value += match_card_value(&plus_card);
                *_draw_pile_numbers+= 1;
                _draw_pile.push(plus_card);
            }
            println!("{}","+2 card played,Player drew two cards".cyan());
        }
        *round_counter += 2;
  } else if matches!(card_to_play, Card::WildP4) {
        if is_player{
            for _ in 0..4 {
                let plus_card = match_card_random();
                *_draw_pile_value += match_card_value(&plus_card);
                *_draw_pile_numbers += 1;
                _draw_pile.push(plus_card);
            }
            println!("{}","+4 card played,bot drew 4 cards".magenta());
            process_wild_card(cards, true);
        }
        else{
            for _ in 0..4 {
                let plus_card = match_card_random();
                *_draw_pile_value += match_card_value(&plus_card);
                *_draw_pile_numbers+= 1;
                _draw_pile.push(plus_card);
            }
            println!("{}","+4 card played,Player drew 4 cards".cyan());
            process_wild_card(cards, false);
        }
        *round_counter += 1;
  } else if matches!(card_to_play, Card::Wild) {
        if is_player{
            process_wild_card(cards, true);
        }
        else{
            process_wild_card(cards, false);
        }
        *round_counter += 1;
  } else {
        *round_counter += 1;
    }

}
use std::{thread, time::Duration};
fn main() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();
    println!("{}","\t\tWelcome from Uno World!".magenta().italic().bold());
    println!("{}","\t\t------------------------".magenta());
    loop {
        let mut time_count = 8;
        let mut end=8;
        let mut turn: i64 = 0;
        println!("{}","\tIf you're ready, press 1 to start the game!".cyan().bold());
        let mut start = String::new();
        io::stdin().read_line(&mut start).expect("Failed to read.");
        let start: i32 = start.trim().parse().unwrap_or(0);
       
        if start == 1 {
            println!("{}","\t\tThe game begins in".magenta().bold());
            println!("{}","\t\t\t3\n".cyan().italic());
            thread::sleep(Duration::from_secs(1)); 
            println!("{}","\t\t\t2\n".cyan().italic());
            thread::sleep(Duration::from_secs(1));
            println!("{}","\t\t\t1\n".cyan().italic());
            thread::sleep(Duration::from_secs(1));
            println!("{}","\t\t\tGO!\n\n".magenta().italic());
            thread::sleep(Duration::from_secs(1));
            stdout.flush().unwrap();
            execute!(stdout, Clear(ClearType::All)).unwrap();

            let mut player_cards = card_random();
            let mut player_card_numbers = player_cards.len();
            let mut player_card_value = card_value(&player_cards);

            let mut bot_cards = card_random();
            let mut bot_card_numbers = bot_cards.len();
            let mut bot_card_value = card_value(&bot_cards);

            let mut fight_card: Card = match_card_random();
            while fight_card == Card::Wild || fight_card == Card::WildP4 {
                fight_card = match_card_random();
            }
            let mut available_cards = available(&fight_card);

            while time_count >= 0 {
                show_card_status(&mut time_count,&mut end,&player_cards,&bot_cards,&player_card_numbers,&bot_card_numbers,&player_card_value,&bot_card_value,);

                if time_count>0{
                    let colored_fight_card = format!("[Fight Card is {:?}]", fight_card).dark_blue().italic();
                    println!("{}   ", colored_fight_card);
                    if fight_card!=Card::Wild && fight_card!=Card::WildP4
                    {
                    available_cards.clear();
                    available_cards=available(&fight_card);
                    }

                    if turn % 2 == 0 {
                        player_turn(&mut available_cards,&mut fight_card,&mut player_cards,&mut player_card_value,&mut player_card_numbers,&mut bot_cards,&mut bot_card_value,&mut bot_card_numbers,&mut turn,);
                  } else {
                        bot_turn(&mut available_cards,&mut fight_card,&mut player_cards,&mut player_card_value,&mut player_card_numbers,&mut bot_cards,&mut bot_card_value,&mut bot_card_numbers,&mut turn,);
                    }
                    }
                    time_count -= 1;

                }
                break;
      } else {
                println!("{}","Invalid choice. Try again!".red());
                continue;
            }
    }
}
