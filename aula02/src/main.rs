#![allow(unused)]

use core::num;
use std::{num::ParseIntError, result};

fn main() {
    let valor_teste1: Option<String> = None;
    let valor_teste2: Option<String> = Some(1.to_string());
    let valor_teste3: Option<String> = Some(String::from("bla"));
    let valor_teste4: Option<String> = Some(String::from("-1"));

    let resultado1 = soma_1(valor_teste1);
    let resultado2 = soma_1(valor_teste2);
    let resultado3 = soma_1(valor_teste3);
    let resultado4 = soma_1(valor_teste4);

    println!("O Resultado 1 é {:?}", resultado1);

    println!("O Resultado 2 é {:?}", resultado2);
    println!("O Resultado 3 é {:?}", resultado3);
    println!("O Resultado 4 é {:?}", resultado4);

    // //String slice
    // let mut texto: &str = "HEllo";
    // //Shadowing
    // let mut texto = texto.to_string();
    // texto.push_str(", how are you?");

    // //String - StringBuffer
    // let mut texto2: String = String::from("Hello");
    // texto2.push_str(" World");

    // println!("{texto}");
    // println!("{texto2}");

    // let um_boolean: bool = "true".parse().unwrap();
    // let resultado_inteiro: Result<i32, ParseIntError> = "123".parse();
    // let um_inteiro = match resultado_inteiro {
    //     Ok(valor) => valor,
    //     Err(err) => -1,
    // };

    // println!("{um_inteiro} {um_boolean}");
}

///Receber uma option com uma string
/// Vai validar se a option esta preenchida, contem um número e o numero não é negativo
/// Vai converter a string para numero, somar um e retornar outra option com o resultado em string
fn soma_1(algum_valor: Option<String>) -> Result<String, String> {
    // if let Some(texto) = algum_valor {
    // let resultado_inteiro: Result<i32, ParseIntError> = texto.parse();
    // let resultado_convertido =
    //     resultado_inteiro.map_err(|parse_int_error| parse_int_error.to_string());

    // let valor_convertido = match resultado_convertido {
    //     Ok(valor) => valor,
    //     Err(erro_de_parse) => {
    //         // return Err(format!(
    //         //     "Erro, provavelmente valor não é número: {}",
    //         //     erro_de_parse.to_string()
    //         // ))
    //         return Err(erro_de_parse);
    //     }
    // };
    // let resultado_inteiro: Result<i32, ParseIntError> = texto.parse();
    // let resultado_convertido = resultado_inteiro.map_err(|parse_error| parse_error.to_string());

    // let valor_convertido: i32 = resultado_convertido?;

    // let resultado_conversao = valida_numero(valor_convertido);
    // let valor_validado = match resultado_conversao {
    //     Err(mensagem) => return Err(mensagem),
    //     Ok(valor) => valor,
    // };

    //Operador ?
    //Chamando-se uma função que retorna result, seguida de um ?
    //O rust vai tentar extrair o valor Ok, se conseguir, retorna o valor de dentro do ok
    //se não conseguir, ele retorna imediatamente da função com Err

    let texto = algum_valor.ok_or("Obrigatorio passar um valor".to_string())?;

    let valor_convertido: i32 = texto
        .parse()
        .map_err(|err: ParseIntError| err.to_string())?;

    let valor_validado: i32 = valida_numero(valor_convertido)?;

    Ok((valor_validado + 1).to_string())
    // } else {
    //     Err("Obrigatorio passar um valor".to_string())
    // }
}

fn valida_numero(numero: i32) -> Result<i32, String> {
    if numero < 0 {
        Err("Numero não pode ser negativo".to_string())
    } else {
        Ok(numero)
    }
}
