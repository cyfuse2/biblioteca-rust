use std::sync::{Arc, Mutex};
use crate::models::{livro::Livro, membro::Membro};

#[derive(Clone)]
pub struct Database {
    livros: Arc<Mutex<Vec<Livro>>>,
    membros: Arc<Mutex<Vec<Membro>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            livros: Arc::new(Mutex::new(Vec::new())),
            membros: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Acesso controlado aos livros
    pub fn get_livros_mutex(&self) -> &Mutex<Vec<Livro>> {
        &self.livros
    }

    // Acesso controlado aos membros
    pub fn get_membros_mutex(&self) -> &Mutex<Vec<Membro>> {
        &self.membros
    }

    // Mantenha os métodos existentes...
    pub fn adicionar_livro(&self, livro: Livro) {
        let mut livros = self.livros.lock().unwrap();
        livros.push(livro);
    }

    pub fn buscar_livros(&self) -> Vec<Livro> {
        self.livros.lock().unwrap().clone()
    }

    pub fn adicionar_membro(&self, membro: Membro) {
        let mut membros = self.membros.lock().unwrap();
        membros.push(membro);
    }

    pub fn buscar_membros(&self) -> Vec<Membro> {
        self.membros.lock().unwrap().clone()
    }
}