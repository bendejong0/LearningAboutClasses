// today i will build a class. my girlfriend has a pigeon, and we both really like her,
// so im making a pigeon class.

use std::io;
//allows inputs

struct Pigeon{
    currentSpeed: f32,
    isWalking: bool,
}

trait PigeonMovement{
    //calculates how quickly the pigeon can speed up
    fn acceleration(&mut self) -> i32{
        
        let changeInSpeed: f32 = 1.2;
        self.currentSpeed=changeInSpeed*self.currentSpeed+0.5;      //giving it a value of .5 assumes that when IS moving it's going at least 0.5 mph
                                                                    //we have to do this or else it'll just mutiply changeInSpeed by 0 over and over;
        return self.currentSpeed;
    }

    //calculates how quickly the pigeon can slow down
    fn deacceleration(&mut self) ->f32{
        
        let pchangeInSpeed: *mut i32 = &mut self.changeInSpeed; //just made it a pointer for the sake of removing some '.self's
        let changeInSpeed: f32 = 0.2;      //the pigeon stops much faster than it accelerates
        pchangeInSpeed=changeInSpeed*pchangeInSpeed;

        if pchangeInSpeed <= 0.1{       //if the pigeon is moving this slowly,
            pchangeInSpeed = 0;         //is it even moving at all?
        }
        return pchangeInSpeed
    }

    fn walking(&mut self)->i32{
        while self.currentSpeed < 2 && self.isWalking {
            self.currentSpeed = self.acceleration(self.currentSpeed);
            println!("Still walking?");
        }
    }
}

//getting the error that Pigeon(f32) is "not a trait".
//I will have to look into this when i can.

fn main() -> Box<dyn PigeonMovement>{
    let mut worky: bool = true;
    let mut str = String::new();
    let mut speed = String::new();
    let mut newspeed: f32;
    while worky{
        io::stdin()
            .read_line(&speed)
            .expect("bruhhhhhh this brokey on line 50");
        newspeed = speed.trim();
        match speed.parse::<f32>(){};
        print!("keep worky?");
        io::stdin()
            .read_line(&mut str)
            .expect("WORKY YES OR NO??????");
        str=str.trim();
        if str == 'y' || str == "Y" || str == "yes" || str == "YES"{
            worky=true;
        }
        else{
            worky=false;
        }

        print!("{} mph", speed)
    }
}
