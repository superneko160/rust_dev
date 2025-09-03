// --- 振る舞い（アルゴリズム）---
// 振る舞いを定義するトレイト
trait FlyBehaviour {
    fn fly(&self);
}

// 飛ぶ
struct FlyWithWings;

impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("i can fly with my wings!");
    }
}

// 飛ばない
struct FlyNoWay;

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("i can't fly.");
    }
}
// --- 振る舞い（アルゴリズム）---


// --- 振る舞い（アルゴリズム）を移譲する主体 ---
// 鳥の共通の振る舞いを定義するトレイト
trait Bird {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour;
    fn fly(&self) {
        let s = self.get_fly_behaviour();
        s.fly();
    }
}

// 鳩の構造体
struct Pigeon {
    fly_behaviour: Box<dyn FlyBehaviour>,
}

impl Bird for Pigeon {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl Pigeon {
    // 鳩を生成
    fn new(fly_behaviour: Box<dyn FlyBehaviour>) -> Self {
        Pigeon { fly_behaviour }
    }
    // 振る舞いを変更
    fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fly_behaviour;
    }
}

// ペンギンの構造体
struct Penguin {
    fly_behaviour: Box<FlyNoWay>,
}

impl Bird for Penguin {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl Penguin {
    // ペンギンを生成
    fn new(fly_behaviour: Box<FlyNoWay>) -> Self {
        Penguin { fly_behaviour }
    }
}
// --- 振る舞い（アルゴリズム）を移譲する主体 ---


pub fn main() {
    // 翼で飛ぶ振る舞いを持つ鳩を生成
    let mut pigeon = Pigeon::new(Box::new(FlyWithWings));
    pigeon.fly();

    // 鳩の振る舞いを、飛ばないように変更
    println!("--- Something is wrong with the pigeon. ---");
    pigeon.set_fly_behaviour(Box::new(FlyNoWay));
    pigeon.fly();

    // 飛ばない振る舞いが固定されたペンギンを生成
    println!("--- Penguin appears. ---");
    let penguin = Penguin::new(Box::new(FlyNoWay));
    penguin.fly();
}
