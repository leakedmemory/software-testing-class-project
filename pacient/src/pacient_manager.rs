use std::io;

use common_lib::io_handler::IOHandler;
use common_lib::json_handler::JsonHandler;
use common_lib::priority_queue::{PriorityQueue, TicketPriority};

pub struct PacientManager<R, W> {
    io_handler: IOHandler<R, W>,
    queue: PriorityQueue,
    queue_path: String,
}

impl<R, W> PacientManager<R, W>
where
    R: io::BufRead,
    W: io::Write,
{
    pub fn new(io_handler: IOHandler<R, W>, queue: PriorityQueue, queue_path: String) -> Self {
        Self {
            io_handler,
            queue,
            queue_path,
        }
    }

    pub fn start(&mut self) -> ! {
        self.io_handler
            .write("Seja bem-vindo(a) à SOS Dentes!\n")
            .expect("Unable to write welcome message");

        loop {
            let ticket_priority = self.get_ticket_priority_input();

            match self.parse_ticket_priority_input(&ticket_priority) {
                Some(priority) => self.handle_enqueue(priority),
                None => {
                    let invalid_input_msg =
                        "\nTipo de atendimento INVÁLIDO. Por favor, insira novamente.\n";

                    self.io_handler
                        .write(invalid_input_msg)
                        .expect("Unable to write invalid input error message");
                }
            }
        }
    }

    fn get_ticket_priority_input(&mut self) -> String {
        self.io_handler
            .write(
                "\n[1] Prioritário\n\
                [2] Normal\n\
                \n\
                Insira o tipo de atendimento para receber seu número de chamada: ",
            )
            .expect("Unable to write priority options");

        self.io_handler
            .read_line()
            .expect("Unable to read priority")
    }

    fn parse_ticket_priority_input(&self, priority: &str) -> Option<TicketPriority> {
        let trimmed = priority.trim();
        if trimmed == "1" {
            return Some(TicketPriority::High);
        } else if trimmed == "2" {
            return Some(TicketPriority::Normal);
        }

        None
    }

    fn handle_enqueue(&mut self, priority: TicketPriority) {
        match self.queue.enqueue(priority) {
            Ok(()) => {
                JsonHandler::save_as_json(&self.queue_path, &self.queue.get_queue())
                    .expect("Unable to save queue in file");
                println!(
                    "\nPedido de atendimento aceito.\n\
                    Você será chamado(a) quando for sua vez. Por favor, aguarde."
                );
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}