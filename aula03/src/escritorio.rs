pub struct Pessoa {
    nome: String,
    idade: u8,
    salario: f64,
}

impl Pessoa {
    pub fn new(nome: String, idade: u8, salario: f64) -> Pessoa {
        Pessoa {
            nome,
            idade,
            salario,
        }
    }

    pub fn idade(&self) -> u8 {
        self.idade
    }

    pub fn nome(&self) -> &str {
        self.nome.as_str()
    }

    pub fn salario(&self) -> f64 {
        self.salario
    }

    pub fn set_salario(&mut self, salario: f64) {
        self.salario = salario;
    }
}

pub fn desconta_falta(pessoa: &mut Pessoa, valor: f64) {
    pessoa.salario -= valor;
}
