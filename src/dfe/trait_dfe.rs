pub trait DFe {
    // return id of a DFe
    fn getId(&self) -> &str;

    // return model of a DFe
    fn getModel(&self) -> DFeType; 

    // return item of dfe
    fn getItem(&self, i32) -> DFeItem;

    // return all itens of a DFe
    fn getItens(&self) -> Itens

    // returm "Emissor" of the dfe
    fn getEmissor(&self) -> emissor;

    // return DFe total value
    fn getTotalAmount(&self) -> f64;

    // return xml file
    fn getXMLString(&self) -> String;
}
