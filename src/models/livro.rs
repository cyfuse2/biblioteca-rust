#[derive(Debug, Clone)]
#[allow(dead_code)] // Adicione esta linha
pub struct Livro {
    pub id: u32,
    pub titulo: String,
    pub autor: String,
    pub disponivel: bool,
}

impl Livro {
    pub fn new(id: u32, titulo: String, autor: String) -> Self {
        Self {
            id,
            titulo,
            autor,
            disponivel: true,
        }
    }
}