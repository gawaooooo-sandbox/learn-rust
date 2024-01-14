// 宝箱の振る舞いを定義するトレイと
trait TreasureBox {
    // デフォルトメソッド
    fn open(&self, key_no: i32) -> bool {
        // 指定の鍵でのみ箱は開く
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

// 宝石箱を定義
struct JewelryBox {
    price: i32,  // 金貨何枚分か
    key_no: i32, // 何番の鍵があれば開くか
}
impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("宝石箱だった！金貨{}枚入手。", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 空箱を定義
struct EmptyBox {
    key_no: i32, // 何番の鍵があれば開くか
}
impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("空箱だった！何も入っていない。");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 冒険者が箱を開ける
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = EmptyBox { key_no: 1 };
    let box3 = JewelryBox {
        price: 50,
        key_no: 2,
    };

    // 冒険者が宝箱を手持ちの鍵で開ける
    open_box(&box1, 1);
    open_box(&box2, 1);
    open_box(&box3, 1);
}
