mod sausage_factory {
    // 私有函数
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 公有函数
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}