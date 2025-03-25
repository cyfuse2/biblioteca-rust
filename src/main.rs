mod models;
mod services;
mod database;

use std::io;
use services::{livro_service::LivroService, membro_service::MembroService};
use database::Database;

fn main() {
    let db = Database::new();
    exibir_menu_principal(&db);
}

fn exibir_menu_principal(db: &Database) {
    loop {
        println!("\n=== Sistema Biblioteca ===");
        println!("1. Adicionar Livro");
        println!("2. Listar Livros");
        println!("3. Adicionar Membro");
        println!("4. Listar Membros");
        println!("5. Emprestar Livro");
        println!("6. Devolver Livro");
        println!("0. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Falha ao ler entrada");
        
        match escolha.trim() {
            "1" => adicionar_livro(db),
            "2" => listar_livros(db),
            "3" => adicionar_membro(db),
            "4" => listar_membros(db),
            "5" => emprestar_livro(db),
            "6" => devolver_livro(db),
            "0" => break,
            _ => println!("Opção inválida!"),
        }
    }
}

// Implementação das funções de interação

fn adicionar_livro(db: &Database) {
    println!("Digite o título do livro:");
    let mut titulo = String::new();
    io::stdin().read_line(&mut titulo).expect("Falha ao ler entrada");
    
    println!("Digite o autor do livro:");
    let mut autor = String::new();
    io::stdin().read_line(&mut autor).expect("Falha ao ler entrada");
    
    let livro = LivroService::novo_livro(
        db,
        titulo.trim(),
        autor.trim()
    );
    
    println!("\nLivro adicionado com sucesso!");
    println!("ID: {}, Título: {}", livro.id, livro.titulo);
}

fn listar_livros(db: &Database) {
    println!("\n=== Lista de Livros ===");
    for livro in LivroService::listar_livros(db) {
        let status = if livro.disponivel { "Disponível" } else { "Emprestado" };
        println!("ID: {} | Título: {} | Autor: {} | Status: {}", 
            livro.id, livro.titulo, livro.autor, status);
    }
}

fn adicionar_membro(db: &Database) {
    println!("Digite o nome do membro:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler entrada");
    
    let membro = MembroService::novo_membro(db, nome.trim());
    
    println!("\nMembro cadastrado com sucesso!");
    println!("ID: {}, Nome: {}", membro.id, membro.nome);
}

fn listar_membros(db: &Database) {
    println!("\n=== Lista de Membros ===");
    for membro in MembroService::listar_membros(db) {
        println!("ID: {} | Nome: {} | Livros emprestados: {}", 
            membro.id, membro.nome, membro.livros_emprestados.len());
    }
}

fn emprestar_livro(db: &Database) {
    println!("Digite o ID do livro:");
    let mut livro_id = String::new();
    io::stdin().read_line(&mut livro_id).expect("Falha ao ler entrada");
    let livro_id: u32 = livro_id.trim().parse().expect("ID inválido");

    println!("Digite o ID do membro:");
    let mut membro_id = String::new();
    io::stdin().read_line(&mut membro_id).expect("Falha ao ler entrada");
    let membro_id: u32 = membro_id.trim().parse().expect("ID inválido");

    // Implementar lógica de empréstimo
    match realizar_emprestimo(db, livro_id, membro_id) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Erro: {}", e),
    }
}

fn devolver_livro(db: &Database) {
    println!("Digite o ID do livro:");
    let mut livro_id = String::new();
    io::stdin().read_line(&mut livro_id).expect("Falha ao ler entrada");
    let livro_id: u32 = livro_id.trim().parse().expect("ID inválido");

    // Implementar lógica de devolução
    match realizar_devolucao(db, livro_id) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Erro: {}", e),
    }
}

// Adicione estas funções no services ou database
fn realizar_emprestimo(db: &Database, livro_id: u32, membro_id: u32) -> Result<String, String> {
    // Acesso direto aos dados sem clonar
    let mut livros = db.get_livros_mutex().lock().unwrap();
    let livro = livros.iter_mut().find(|l| l.id == livro_id)
        .ok_or("Livro não encontrado")?;

    if !livro.disponivel {
        return Err("Livro já emprestado".into());
    }

    let mut membros = db.get_membros_mutex().lock().unwrap();
    let membro = membros.iter_mut().find(|m| m.id == membro_id)
        .ok_or("Membro não encontrado")?;

    livro.disponivel = false;
    membro.livros_emprestados.push(livro_id);
    
    Ok(format!("Livro '{}' emprestado para {}", livro.titulo, membro.nome))
}

fn realizar_devolucao(db: &Database, livro_id: u32) -> Result<String, String> {
    let mut livros = db.get_livros_mutex().lock().unwrap();
    let livro = livros.iter_mut().find(|l| l.id == livro_id)
        .ok_or("Livro não encontrado")?;

    if livro.disponivel {
        return Err("Livro já está disponível".into());
    }

    livro.disponivel = true;
    
    let mut membros = db.get_membros_mutex().lock().unwrap();
    for membro in membros.iter_mut() {
        membro.livros_emprestados.retain(|&id| id != livro_id);
    }
    
    Ok(format!("Livro '{}' devolvido com sucesso", livro.titulo))
}