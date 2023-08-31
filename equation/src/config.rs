const ADDER_ADDR: &str = "ADDER_ADDR";
const SUBTRACTOR_ADDR: &str = "SUBTRACTOR_ADDR";
const MULTIPLIER_ADDR: &str = "MULTIPLIER_ADDR";
const DIVIDER_ADDR: &str = "DIVIDER_ADDR";


#[derive(Clone, Debug)]
pub struct Config {
    pub adder_addr: String,
    pub subtractor_addr: String,
    pub multiplier_addr: String,
    pub divider_addr: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Self {
            adder_addr: dotenv::var(ADDER_ADDR).expect("ENVAR present"),
            subtractor_addr: dotenv::var(SUBTRACTOR_ADDR).expect("ENVAR present"),
            multiplier_addr: dotenv::var(MULTIPLIER_ADDR).expect("ENVAR present"),
            divider_addr: dotenv::var(DIVIDER_ADDR).expect("ENVAR present"),
        }
    }
}
