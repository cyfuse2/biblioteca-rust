use crate::{
    database::Database,  // Agora vai encontrar corretamente
    models::membro::Membro
};

// Restante do código permanece igual

pub struct MembroService;

impl MembroService {
    pub fn novo_membro(db: &Database, nome: &str) -> Membro {
        let membros = db.buscar_membros();
        let novo_id = membros.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        let membro = Membro::new(novo_id, nome.to_string());
        db.adicionar_membro(membro.clone());
        membro
    }

    pub fn listar_membros(db: &Database) -> Vec<Membro> {
        db.buscar_membros()
    }
}