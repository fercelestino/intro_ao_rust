#![allow(unused)]
mod escritorio;

use crate::escritorio::{desconta_falta, Pessoa};

fn main() {
    execute();
}

fn execute() {
    let mut funcionario1 = Pessoa::new("Fulano".to_string(), 30, 1000.0);
    let mut funcionario2 = Pessoa::new("Sicrano".to_string(), 30, 1000.0);

    let funcionario_do_mes = &funcionario1;
    println!("O funcionario do mes é {}", funcionario_do_mes.nome());

    let funcionarios_faltantes: [&mut Pessoa; 2] = [&mut funcionario1, &mut funcionario2];

    for funcionario in funcionarios_faltantes {
        desconta_falta(funcionario, 50.0);
    }

    println!("{} {:?}", funcionario1.nome(), funcionario1.salario());
    println!("{} {:?}", funcionario2.nome(), funcionario2.salario());
}
