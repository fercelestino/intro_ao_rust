#![allow(unused)]
mod escritorio;

use std::collections::HashMap;

use escritorio::{Assalariado, Beneficio, DadosFuncionais, DadosPessoais};

use crate::escritorio::{desconta_falta, FuncionarioRegular, Gerente};

fn main() {
    execute();
    //arrays_vs_vectors_vs_tuples();
}

fn informa_funcionario_do_mes() -> i32 {
    //Leu do banco de dados, aplicou regras para enquadramento
    123
}

fn informa_funcionarios_faltantes() -> Vec<i32> {
    vec![123, 321]
}

fn execute() {
    let funcionario1 = FuncionarioRegular::new(
        DadosPessoais::new("Fulano".to_string(), 30),
        DadosFuncionais::new(1000.0),
        Beneficio::new(100.0, 100.0),
    );
    let funcionario2 = FuncionarioRegular::new(
        DadosPessoais::new("Sicrano".to_string(), 30),
        DadosFuncionais::new(1000.0),
        Beneficio::new(100.0, 100.0),
    );

    let id_f1 = 123;
    let id_f2 = 321;

    let mut registros = HashMap::new();
    registros.insert(id_f1, funcionario1);
    registros.insert(id_f2, funcionario2);

    let id_funcionario_do_mes = informa_funcionario_do_mes();

    if let Some(funcionario_do_mes) = registros.get(&id_funcionario_do_mes) {
        println!(
            "O funcionario do mes é {}",
            funcionario_do_mes.dados_pessoais().nome()
        );
    }

    if let Some(funcionario_do_mes) = registros.get_mut(&id_funcionario_do_mes) {
        da_aumento(funcionario_do_mes, 200.0);
    }

    let ids_funcionarios_faltantes = informa_funcionarios_faltantes();
    for id in ids_funcionarios_faltantes.iter() {
        if let Some(funcionario_faltante) = registros.get_mut(id) {
            desconta_falta(funcionario_faltante, 50.0);
        }
    }

    if let Some(funcionario1) = registros.get_mut(&id_f1) {
        da_aumento(funcionario1, 300.0);
    }

    //let mut registros_iterator = registros.values();
    // let primeiro_registro = registros_iterator.next();
    // let segundo_elemento = registros_iterator.next();
    // let nao_vai_ter_mais_nada = registros_iterator.next();

    // println!("Primeiro {:?}", primeiro_registro);
    // println!("Segundo {:?}", segundo_elemento);
    // println!("nao tem {:?}", nao_vai_ter_mais_nada);

    // while let Some(pessoa) = registros_iterator.next() {
    //     println!(
    //         "O nome é {}, e o salario é {}",
    //         pessoa.nome(),
    //         pessoa.salario()
    //     );
    // }

    for pessoa in registros.values() {
        println!(
            "O nome é {}, e o salario é {}",
            pessoa.dados_pessoais().nome(),
            pessoa.dados_funcionais().salario()
        );
    }
}

pub fn da_aumento<T: Assalariado>(pessoa: &mut T, valor: f64) {
    let mut dados_funcionais = pessoa.dados_funcionais_mut();

    dados_funcionais.set_salario(dados_funcionais.salario() + valor);
}

fn arrays_vs_vectors_vs_tuples() {
    let int_array: [i32; 3] = [1, 2, 3]; //STACK

    let segundo_elemento_do_array = int_array[1];

    let mut int_vector: Vec<i32> = vec![1, 2, 3]; //HEAP - MEMORY LEAKS - gerenciamento de memoria e garbage collector em outras linguagens
                                                  // let mut int_vector: Vec<i32> = Vec::new();
                                                  // int_vector.push(1);
                                                  // int_vector.push(2);
                                                  // int_vector.push(3);

    let algum_elemento_do_vector = int_vector.get(2);
    if let Some(elemento) = algum_elemento_do_vector {
        println!("O elemento é {}", elemento);
    }
    println!("O elemento de indice 2 é {algum_elemento_do_vector:?}");

    let uma_tuple: (i32, i32) = (1, 2);
    println!("primeiro da tuple é {}", uma_tuple.0);
    println!("segundo da tuple é {}", uma_tuple.1);

    let outra_tuple: (&str, i32) = ("Hello", 123);

    let uma_pessoa: (i64, &str, u8, &str) = (12312312312, "Fulano", 30, "Rua das Flores");
}
