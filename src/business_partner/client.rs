use crate::business_partner::trait_business_partner::BusinessPartner;
use crate::business_partner::business_partner_type::BusinessPartnerType;

pub struct Client {
    pub id: u64,
    pub name: String, 
    pub cnpj: String, 
}

impl BusinessPartner for Client {
    // return id
    fn get_id(&self) -> u64{
        self.id        
    }
    
    //return name
    fn get_name(&self) -> &str {
        &self.name
    }

    // return cnpj
    fn get_document(&self) -> &str {
        &self.cnpj        
    }

    // return type of business partner
    fn get_type(&self) -> BusinessPartnerType {
        BusinessPartnerType::Client
    }
}
