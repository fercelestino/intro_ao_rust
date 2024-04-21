#![allow(unused)]
fn main() {
    let posso_existir_ou_nao: Option<i32> = Some(10);

    let posso_existir_2: Option<i32> = None;

    // match posso_existir_ou_nao {
    //     Some(val) => {
    //         let resultado = soma_1(val);
    //         println!("o resultado da soma é {resultado}");
    //     }
    //     None => println!("Não tinha nada"),
    // }

    // match posso_existir_2 {
    //     Some(val) => println!("O resultado é {}", soma_1(val)),
    //     None => println!("Não tinha nada"),
    // }

    // if let Some(val) = posso_existir_ou_nao {
    //     println!("O valor é {val}");
    // } else {
    //     println!("Não tinha nada");
    // }

    // if let Some(val) = posso_existir_2 {
    //     println!("O valor é {val}");
    // } else {
    //     println!("Não tinha nada");
    // }

    // if posso_existir_ou_nao.is_some() {
    //     let val = posso_existir_ou_nao.expect("Já foi checado e tem um valor");
    //     println!("A soma é {}", soma_1(val));
    // }

    // if posso_existir_2.is_some() {
    //     let val = posso_existir_2.expect("Já foi checado e tem um valor");
    //     println!("A soma é {}", soma_1(val));
    // }

    // let mut resultado: Option<i32> = None;
    // if let Some(val) = posso_existir_ou_nao {
    //     resultado = Some(soma_1(val));
    // }

    //JavaScript arrow function
    // let minhafuncao = (val) => {
    //     let result = val + 1;
    //     return result;
    // }
    // //Closure no rust
    // let minhaclosure = |val| {
    //     let result = val + 1;
    //     return result;
    // }

    // let resultado = posso_existir_2.map(|val| val + 1);

    // let minhaclosure = |val| soma_1(val);

    let resultado = posso_existir_ou_nao.map(soma_1);

    println!("O resultado é {resultado:?}");
}

fn soma_1(algum_valor: i32) -> i32 {
    println!("entrei na funcao");

    if algum_valor == -1 {
        println!("nao faço com negativos");
        return -1;
    }
    //varias outras ops
    algum_valor + 1
}
