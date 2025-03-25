#[derive(Debug, Clone)]
#[allow(dead_code)] // Adicione esta linha
pub struct Membro {
    pub id: u32,
    pub nome: String,
    pub livros_emprestados: Vec<u32>,
}

impl Membro {
    pub fn new(id: u32, nome: String) -> Self {
        Self {
            id,
            nome,
            livros_emprestados: Vec::new(),
        }
    }
}