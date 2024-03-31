use std::fs::File; 
use std::io::*;
use std::path::Path;
use std::fs;
use std::collections::HashMap;


struct Question{

    question:String,
    wrong_answer_1:String,
    wrong_answer_2:String,
    wrong_answer_3:String,
    wrong_answer_4:String,
    right_answer:String,
}

impl Question{

    fn new(question:String,wrong_answer_1:String,wrong_answer_2:String,wrong_answer_3:String,wrong_answer_4:String,right_answer:String)->Self{

        Self{
            question,
            wrong_answer_1,
            wrong_answer_2,
            wrong_answer_3,
            wrong_answer_4,
            right_answer,
        }
    }
}

struct Game{

    score: u32,
    streak: u32,
    num_questions:u32,
    question_index:u32,
    curr_user_guess:String,
    questions:Vec<Question>,
    done:bool,
    question_genre:u32,
    question_files_map:HashMap<u32,String>,
    question_topic_names:Vec<String>,

}

impl Game{

    fn new()-> Self {

        Self{

            questions:Vec::new(),
            score:0,
            streak:0,
            num_questions:0,
            question_index:0,
            curr_user_guess:String::from(""),
            done:false,
            question_genre:0,
            question_files_map:HashMap::new(),
            question_topic_names:Vec::new(),
        }

    }

    fn init_data(&mut self ){
        self.get_question_files();
    }

    fn get_category_from_file(&mut self, f:String) -> String{
        let fileName = f;
        let parts = fileName.split("_");
        let mut finalGenre = String::from("");
        
        for p in parts {
            let tempP:&str = &p;
            if tempP.contains(".data"){
                let genre_parts = tempP.split(".");
                for gPart in genre_parts{
                    if !gPart.contains("data"){
                        finalGenre = (&gPart).to_string();
                    }
                }
            }
        }
        
        finalGenre
    }

    fn get_question_files(&mut self){
        
        let directory = fs::read_dir("./data").unwrap();

        let mut index = 0;
        for file in directory {
            let tFile = file.as_ref();
            let ttFile = file.as_ref();
            let fileName = tFile.as_ref().unwrap().path();
            let data = fs::read_to_string(fileName).unwrap();
            let cat_name = self.get_category_from_file(ttFile.as_ref().unwrap().path().display().to_string());
            //add to hashmap
            println!("loading: file# {}.....",index);
            index +=1;
            self.question_files_map.insert(index,data);
            self.question_topic_names.push(cat_name);

        }

    }

    fn load_questions(&mut self) {
        
        if self.done {
            return 
        } 

        let mut res = false;

        let data = self.question_files_map[&self.question_genre].as_str();

        let question_strings = data.split("\n");
        for string in question_strings {

            let question_parts = string.split("|").collect::<Vec<&str>>();
            
            if question_parts.len() > 1 {
            let temp_question = question_parts[0];
            let temp_answer1 = question_parts[1];
            let temp_answer2 = question_parts[2];
            let temp_answer4 = question_parts[3];
            let temp_answer3 = question_parts[4];
            let temp_c_answer = question_parts[5];

            let mut tempQuestion = Question::new(temp_question.to_string(),temp_answer1.to_string(),temp_answer2.to_string(),temp_answer3.to_string(),temp_answer4.to_string(),temp_c_answer.to_string());

            self.questions.push(tempQuestion);
            }

        }
    }

    fn main_loop(& mut self){
        loop {


            if self.done {
                break
            }
            else{
                self.read_question()
            }
        }
    }

    fn increment_stats(&mut self){
        self.score+=1;
        self.streak+=1;
    }

    fn reset_stats(&mut self){
        self.streak=0;
    }

    fn read_question(& mut self){

        let question = &self.questions[self.question_index as usize];
        let mut temp_correct_index = String::from("");
        println!("Score: {} | Streak: {}",self.score,self.streak);
        println!("{}",question.question);


        println!("1- {}",question.wrong_answer_1);
        println!("2- {}",question.wrong_answer_2);
        println!("3- {}",question.wrong_answer_3);
        println!("4- {}",question.wrong_answer_4);
        temp_correct_index = question.right_answer.clone();


        let answer = self.get_user_input();

        if answer.trim() == temp_correct_index.trim() {
            println!("Correct!");
            self.increment_stats()
        }
        else{
            println!("Wrong! The correct answer was: {}",question.right_answer);
            self.reset_stats()
        }

        self.question_index+=1;

        if self.questions.len() as u32 > 0 && self.question_index >=self.questions.len() as u32 {
            self.done = true;
            self.question_index=0;
        }


    }

    fn get_user_input(&self)->String{
        let mut s = String::from("");
        
        stdin().read_line(&mut s).expect("unable to read input!");

        s.trim().to_lowercase()
    }
}



fn main() {
    let mut game = Game::new();
    game.init_data();

    println!("Welcome to quiz app. Get Ready the Questions!");
    println!("Press any key to start!");

    let input = game.get_user_input();

    loop{
        println!("Enter the number of the question category you want or [e]xit to quit");
        println!("***************************************************");
        let mut menu_index=0;
        for entry in &game.question_topic_names{
            menu_index+=1;
            println!("Press {} for - {} ",menu_index,entry);
        }
        println!("***************************************************");
        let q_index = game.get_user_input();
        match q_index.as_str() {
            "1" => {
                game.question_genre = 1
            },
            "2" => {
                game.question_genre = 2
            },
            "3" => {
                game.question_genre = 3
            },
            "4" => {
                game.question_genre = 4
            },
            "5" => {
                game.question_genre = 5
            }
            "6" => {
                game.question_genre = 6
            },
            "7" => {
                game.question_genre = 7
            },
            "exit" => {
                game.done= true;
            }
            _ => {
                println!("Invalid entry");
                
            }
        }

        if game.question_genre > 0 || game.done == true {
            break
        }
    }
    
    game.load_questions();
    game.main_loop();


}
