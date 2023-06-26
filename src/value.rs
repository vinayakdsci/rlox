// Representing values in the VM requires a struct to hold them
//


pub type Value = f64;

#[derive(Debug, Clone)]
pub struct ValueArray {
    pub count: usize,
    pub values: Vec<Value>,  // all the numerical values are float under the hood
}


impl ValueArray {
    pub fn init_value_array() -> Self {
        let val_arr = Vec::<Value>::with_capacity(8);
        Self {
            count: 0,
            values: val_arr,
        }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
        self.count += 1;
    }
}



