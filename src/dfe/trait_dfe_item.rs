pub trait DFeItem {
    // return item number
    fn GetItemNumber(&self) -> u32; 

    // return product code
    fn GetCProd(&self) -> &str; 

    // return unit sales value
    fn GetVUnCom(&self) -> f64; 

    // return Gross value of the product
    fn GetVProd(&self) -> f64; 

    // return Commercial Quantity
    fn GetQCom(&self) -> f4
}
