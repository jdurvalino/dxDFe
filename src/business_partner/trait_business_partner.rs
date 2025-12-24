use crate::business_partner::business_partner_type::BusinessPartnerType;

pub trait BusinessPartner {
    
    // Return business partner Id
    fn get_id(&self) -> u64;

    // return razão social
    fn get_name(&self) -> &str;

    // Documento (CPF ou CNPJ)
    fn get_document(&self) -> &str;

    // return business partner type (Cliente or Supplier)
    fn get_type(&self) -> BusinessPartnerType;
}

