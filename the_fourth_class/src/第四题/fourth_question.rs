use std::thread;
use std::time::Duration;

//第四题
fn main() {
    //创建一个 红绿灯 集合
    let v = vec![TrafficLight::Yellow, TrafficLight::Green, TrafficLight::Red];
    //遍历集合 处理每一个红绿灯
    v.into_iter().for_each(handle_traffic_light);
}

//处理红绿灯的方法
fn handle_traffic_light(light: TrafficLight) {
    //根据红绿灯的颜色 打印不同的信息 并且为每一个红绿灯延迟
    thread::sleep(Duration::from_secs(light.return_time()));
}

//枚举 红绿灯
enum TrafficLight {
    Red,       //红灯
    Yellow,     //黄灯
    Green       //绿灯
}

// 返回时间的trait
trait ReturnTime {
    //返回时间
    fn return_time(&self) -> u64;
}

// 实现返回时间的trait
impl ReturnTime for TrafficLight {
    fn return_time(&self) -> u64 {
        match self {
            // 红灯
            TrafficLight::Red => {
                println!("Red light");
                3
            },
            // 黄灯
            TrafficLight::Yellow => {
                println!("Yellow light");
                1
            },
            // 绿灯
            TrafficLight::Green => {
                println!("Green light");
                2
            }
        }
    }
}

