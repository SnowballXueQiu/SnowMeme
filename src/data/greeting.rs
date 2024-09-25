use chrono::Timelike;

pub fn get_greeting() -> String {
    let time = chrono::Local::now().time();

    let hour = time.hour();
    
    if hour == 7 {
        return "早上好，今天是充满希望的一天！".to_string();
    } else if hour == 12 {
        return "中午好，愿你的午餐愉快，继续加油！".to_string();
    } else if hour == 15 {
        return "下午好，保持积极，迎接接下来的挑战！".to_string();
    } else if hour == 18 {
        return "晚上好，放松心情，享受美好的时光！".to_string();
    } else if hour == 22{
        return "晚安，今天辛苦了，愿你有个好梦！".to_string()
    }
    
    "额啊？！现在是几点？好像时间有点错乱（请@管理员）".to_string()
}
