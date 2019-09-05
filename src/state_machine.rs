#![feature(unicode_internals)]

#[derive(Clone, Debug)]
struct CandyMachine {
    coins: u32,
    candies: u32,
    locked: bool
}

fn insert_coin(cm: CandyMachine) -> Result<CandyMachine, String> {
    println!("Inserting coin...");
    match cm.locked {
        true if cm.candies > 0 =>
            Ok(CandyMachine {
                coins: cm.coins + 1,
                locked: false,
                ..cm
            }),
        true => {
            Err(format!("No more candies available!"))
        },
        false => {
            Err(format!("Machine is already unlocked!"))
        }
    }
}

fn turn_knob(cm: CandyMachine) -> Result<CandyMachine, String> {
    println!("Turning knob...");
    match cm.locked {
        false if cm.candies > 0 =>
            Ok(CandyMachine {
                candies: cm.candies - 1,
                locked: true,
                ..cm
            }),
        false => {
            Err(format!("No more candies available!"))
        },
        true => {
            Err(format!("Machine is locked!"))
        }
    }
}

fn check_res(res: Result<CandyMachine, String>) -> CandyMachine {
    match res {
        Ok(cm) => {
            println!("Candy Machine state: {} coins, {} candies, locked? [{}]",
                     cm.coins, cm.candies, cm.locked);
            cm
        },
        Err(e) => {
            panic!("Failed: {:?}", e);
        }
    }
}

fn check_res_err(res: Result<CandyMachine, String>) {
    match res {
        Err(e) => {
            println!("Error: {:?}", e);
        },
        Ok(_) => {
            panic!("Shouldn't have worked!!");
        }
    }
}

fn main() {
    let cm = CandyMachine {
        coins: 0,
        candies: 2,
        locked: true
    };

    check_res_err(turn_knob(cm.clone()));
    let cm = check_res(insert_coin(cm));
    check_res_err(insert_coin(cm.clone()));
    let cm = check_res(turn_knob(cm));
    let cm = check_res(insert_coin(cm));
    let cm = check_res(turn_knob(cm));
    check_res_err(insert_coin(cm.clone()));
    check_res_err(turn_knob(cm.clone()));

}
