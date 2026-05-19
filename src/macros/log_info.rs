#[macro_export]

macro_rules! log_info {
    ($info: expr) => {
        println!("[file]:{}, [line]:{}, [info]: {}",
            file!(), line!(), $info);
    }
}