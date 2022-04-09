//第六题
fn main() {
    let circle = Graphic::Circle(3.0);
    print_area(circle);

    let rectangle = Graphic::Rectangle { width: 3.0, height: 4.0 };
    print_area(rectangle);

    let triangle = Graphic::Triangle { bottom: 3.0, height: 4.0 };
    print_area(triangle);
}

//打印面积的方法
fn print_area<T: CalcArea>(graphic: T) {
    println!("area = {}", graphic.calc_area());
}

//计算面积的trait
trait CalcArea {
    // 计算面积的方法
    fn calc_area(&self) -> f64;
}

//图形枚举体
enum Graphic {
    //三角形
    Triangle {
        bottom: f64,
        height: f64,
    },
    //矩形
    Rectangle {
        width: f64,
        height: f64,
    },
    //圆形
    Circle(f64),
}

//实现计算面积
impl CalcArea for Graphic {
    fn calc_area(&self) -> f64 {
        match self {
            Graphic::Triangle { bottom, height } => bottom * height / 2.0,
            Graphic::Rectangle { width, height } => width * height,
            Graphic::Circle(radius) => radius.powf(2.0) * 3.14,
        }
    }
}