use iced::{
    widget::{button, column, container, pick_list, row, text, text_input, Column, Scrollable},
    Alignment, Application, Command, Element, Length, Theme,
};
use crate::{database::Database, services::{livro_service::LivroService, membro_service::MembroService}};

#[derive(Debug, Clone)]
pub enum Message {
    TabSelecionada(Tab),
    TituloLivroAlterado(String),
    AutorLivroAlterado(String),
    NomeMembroAlterado(String),
    AdicionarLivro,
    AdicionarMembro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Livros,
    Membros,
}

#[derive(Debug, Clone)]
pub struct BibliotecaGUI {
    db: Database,
    tab_atual: Tab,
    input_titulo: String,
    input_autor: String,
    input_nome_membro: String,
    livros: Vec<crate::models::livro::Livro>,
    membros: Vec<crate::models::membro::Membro>,
}

// Implementação do método atualizar_listas
impl BibliotecaGUI {
    fn atualizar_listas(&mut self) {
        self.livros = LivroService::listar_livros(&self.db);
        self.membros = MembroService::listar_membros(&self.db);
    }
}

impl Application for BibliotecaGUI {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Database;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let mut app = Self {
            db: flags,
            tab_atual: Tab::Livros,
            input_titulo: String::new(),
            input_autor: String::new(),
            input_nome_membro: String::new(),
            livros: Vec::new(),
            membros: Vec::new(),
        };
        app.atualizar_listas();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sistema de Biblioteca")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TabSelecionada(tab) => {
                self.tab_atual = tab;
            }
            Message::TituloLivroAlterado(titulo) => {
                self.input_titulo = titulo;
            }
            Message::AutorLivroAlterado(autor) => {
                self.input_autor = autor;
            }
            Message::NomeMembroAlterado(nome) => {
                self.input_nome_membro = nome;
            }
            Message::AdicionarLivro => {
                if !self.input_titulo.is_empty() && !self.input_autor.is_empty() {
                    LivroService::novo_livro(&self.db, &self.input_titulo, &self.input_autor);
                    self.input_titulo.clear();
                    self.input_autor.clear();
                    self.atualizar_listas();
                }
            }
            Message::AdicionarMembro => {
                if !self.input_nome_membro.is_empty() {
                    MembroService::novo_membro(&self.db, &self.input_nome_membro);
                    self.input_nome_membro.clear();
                    self.atualizar_listas();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let tab_seletor = row![
            pick_list(
                &Tab::ALL[..],
                Some(self.tab_atual),
                Message::TabSelecionada
            )
            .width(200)
            .padding(10)
        ];

        let conteudo = match self.tab_atual {
            Tab::Livros => self.view_livros(),
            Tab::Membros => self.view_membros(),
        };

        container(column![tab_seletor, conteudo])
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}

impl Tab {
    const ALL: [Tab; 2] = [Tab::Livros, Tab::Membros];
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tab::Livros => "Livros",
                Tab::Membros => "Membros",
            }
        )
    }
}

impl BibliotecaGUI {
    fn view_livros(&self) -> Element<Message> {
        let inputs = row![
            text_input("Título", &self.input_titulo)
                .on_input(Message::TituloLivroAlterado)
                .padding(10),
            text_input("Autor", &self.input_autor)
                .on_input(Message::AutorLivroAlterado)
                .padding(10),
            button("Adicionar")
                .on_press(Message::AdicionarLivro)
                .padding(10)
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let listagem = Scrollable::new(
            Column::with_children(
                self.livros
                    .iter()
                    .map(|livro| {
                        text(format!(
                            "{} - {} ({})",
                            livro.titulo,
                            livro.autor,
                            if livro.disponivel { "Disponível" } else { "Emprestado" }
                        ))
                        .size(16)
                        .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(10)
        )
        .height(Length::Fill);

        column![inputs, listagem]
            .spacing(20)
            .padding(10)
            .into()
    }

    fn view_membros(&self) -> Element<Message> {
        let inputs = row![
            text_input("Nome do Membro", &self.input_nome_membro)
                .on_input(Message::NomeMembroAlterado)
                .padding(10),
            button("Adicionar")
                .on_press(Message::AdicionarMembro)
                .padding(10)
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let listagem = Scrollable::new(
            Column::with_children(
                self.membros
                    .iter()
                    .map(|membro| {
                        text(format!(
                            "{} - {} livros emprestados",
                            membro.nome,
                            membro.livros_emprestados.len()
                        ))
                        .size(16)
                        .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(10)
        )
        .height(Length::Fill);

        column![inputs, listagem]
            .spacing(20)
            .padding(10)
            .into()
    }
}