mod business_partner;

use business_partner::client::Client;
use business_partner::supplier::Supplier; 
use business_partner::trait_business_partner::BusinessPartner; 

use crossterm::terminal;
use std::io;

fn print_business_partner(p: &dyn BusinessPartner) {
    println!(
        "ID: {} | Name: {} | Document: {} | Type: {:?}",
        p.get_id(),
        p.get_name(),
        p.get_document(), 
        p.get_type()
    );
}

fn get_user_input() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.chars().next().unwrap()
}

fn print_line() {
     let (cols, _) = terminal::size().expect("Erro ao obter tamanho do terminal");
    
    // Insert carcters '_' acordin screen size
    let linha = "_".repeat(cols as usize);
    println!("{}", linha);
    println!("");
}

fn print_input_option(){

    let (cols, _) = terminal::size().unwrap();
    let texto = " Seleciona a opção deseja: 1 - Inserir fornecedor 2 - Inserir cliente";
    let largura_texto = texto.chars().count();

    let padding_total = cols as usize - largura_texto;
    let padding_esq = padding_total / 2;
    let padding_dir = padding_total - padding_esq;

    let linha = format!(
        "{}{}{}",
        " ".repeat(padding_esq),
        texto,
        " ".repeat(padding_dir),
    );

    println!("{}", linha);
}

fn print_title() {

    let (cols, _) = terminal::size().unwrap();

    let texto = " dxDFe - Documentos Fiscais Eletrônicos ";
    let largura_texto = texto.chars().count();

    let padding_total = cols as usize - largura_texto;
    let padding_esq = padding_total / 2;
    let padding_dir = padding_total - padding_esq;

    let linha = format!(
        "{}{}{}",
        " ".repeat(padding_esq),
        texto,
        " ".repeat(padding_dir),
    );

    println!("{}", linha);

}


fn main() {
    
    // recovery screen size
    let (cols, _) = terminal::size().expect("Erro ao obter tamanho do terminal");
    
    // Insert carcters '_' acordin screen size
    let linha = "_".repeat(cols as usize);
    println!("{}", linha);
    println!("");

    print_title(); 
    print_input_option(); 
    print_line(); 

    let client = Client {
        id: 1,
        name: "Empresa ABC".to_string(),
        cnpj: "12345678000199".to_string(),
    };

    let supplier = Supplier {
        id: 2,
        name: "Fornecedor XYZ".to_string(),
        cnpj: "98765432000188".to_string(),
    };

    print_business_partner(&client);
    print_business_partner(&supplier);
}

