use std::io;

use crate::input_handler::InputHandler;
use crate::queues_collection::PacientQueue;

pub struct PacientManager;

impl PacientManager {
    pub fn start(queue_file_path: &'static str) {
        println!("Modo 'Paciente' começando...\n");

        let mut queue = PacientQueue::new(queue_file_path);
        loop {
            let ticket_priority =
                match InputHandler::select_ticket_priority(io::stdin().lock(), io::stdout()) {
                    Some(priority) => priority,
                    None => {
                        println!("Tipo de atendimento INVÁLIDO. Por favor, insira novamente.\n");
                        continue;
                    }
                };

            match queue.take_ticket(ticket_priority) {
                Some(ticket_code) => {
                    let people_ahead = queue.get_amount_people_ahead(ticket_priority);

                    println!(
                        "Seu número de chamada para atendimento é {}.\n\
                        Há um total de {} pessoas na sua frente.",
                        ticket_code, people_ahead
                    );
                }
                None => {
                    println!("Não há vagas no momento.");
                }
            }
            println!("Por favor, aguarde.\n");
        }
    }
}