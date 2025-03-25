use crate::{
    database::Database,  // Agora vai encontrar corretamente
    models::livro::Livro
};

// Restante do código permanece igual
pub struct LivroService;

impl LivroService {
    pub fn novo_livro(db: &Database, titulo: &str, autor: &str) -> Livro {
        let livros = db.buscar_livros();
        let novo_id = livros.iter().map(|l| l.id).max().unwrap_or(0) + 1;
        let livro = Livro::new(novo_id, titulo.to_string(), autor.to_string());
        db.adicionar_livro(livro.clone());
        livro
    }

    pub fn listar_livros(db: &Database) -> Vec<Livro> {
        db.buscar_livros()
    }
}