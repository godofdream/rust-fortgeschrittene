

trait PrimeNumber {
    type Output;
    type Input;
    fn is_prime(&self) -> Self::Output {
        Self::Output
    }
    #[inline(always)]
    fn pi(&self) -> f64 {
        self.default_pi()
    }
    #[doc(hidden)]
    fn default_pi(&self) -> f64 {
        3.14159265359
    }

    fn blub(blub: Self::Input) -> Self::Output;
}

impl PrimeNumber for i32 {
    type Output = bool;
    fn pi(&self) -> f64 {
        self.default_pi()
    }
    fn is_prime(&self) -> Self::Output {
        if *self <= 1 {
            return false;
        }
        for i in 2..*self {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }

    
}

impl PrimeNumber for f64{
    type Output = i32;
    fn pi(&self) -> f64 {
        self.default_pi()
    }
    fn is_prime(&self) -> i32 {
        1
    }
    
}

impl<T> PrimeNumber for T where T: num::Integer {
    type Output = bool;
    fn pi(&self) -> f64 {
        self.default_pi()
    }
    fn is_prime(&self) -> Output {
        if *self <= 1 {
            return false;
        }
        for i in 2..*self {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }
}

struct Point<T,U> 
where
    T: PrimeNumber,
    U: Copy
{
    x: T,
    y: U,
}

impl<T: PrimeNumber,U: Copy> Point<T,U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(3.15, 6.30);

    println!(
        "Integer Point: x = {}, y = {}",
        integer_point.get_x(),
        integer_point.get_y()
    );
    println!(
        "Float Point: x = {}, y = {}",
        float_point.get_x(),
        float_point.get_y()
    );
}
