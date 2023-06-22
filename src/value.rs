// Representing values in the VM requires a struct to hold them
//


pub type Value = f32;

#[derive(Debug)]
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
        if self.values.capacity() < self.count + 1 || self.values.is_empty() {
            self.values.push(value);
        }
        else {
            self.values[self.count] = value;
        }
        self.count += 1;
    }
}



