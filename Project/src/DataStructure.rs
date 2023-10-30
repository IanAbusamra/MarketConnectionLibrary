use crate::DataPacket;

pub struct DataStructure {
    size: i32,
    storage: Vec<DataPacket>,
}

impl DataStructure {
    pub fn new() -> Self {
        DataStructure {
            size: 0,
            storage: Vec::new(),
        }
    }

    pub fn add_datapacket(dp: DataPacket) {
        vec.push(dp);
        size = size + 1;
    }

    pub fn print_ds() {
        for dp in &ds {
            //not finished
            print!("{} ", dp);
        }
    }
}