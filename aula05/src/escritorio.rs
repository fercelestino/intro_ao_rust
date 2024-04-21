#[derive(Debug)]
pub struct Beneficio {
    vale_alimentacao: f64,
    fgts: f64,
}

impl Beneficio {
    pub fn new(vale_alimentacao: f64, fgts: f64) -> Self {
        Self {
            vale_alimentacao,
            fgts,
        }
    }

    pub fn vale_alimentacao(&self) -> f64 {
        self.vale_alimentacao
    }

    pub fn fgts(&self) -> f64 {
        self.fgts
    }

    pub fn calcula_beneficios(&self) -> f64 {
        self.fgts + self.vale_alimentacao
    }
}

#[derive(Debug)]
pub struct DadosPessoais {
    nome: String,
    idade: u8,
}

impl DadosPessoais {
    pub fn new(nome: String, idade: u8) -> Self {
        Self { nome, idade }
    }

    pub fn nome(&self) -> &str {
        self.nome.as_str()
    }

    pub fn idade(&self) -> u8 {
        self.idade
    }
}

#[derive(Debug, Clone)]
pub struct DadosFuncionais {
    salario: f64,
}

impl DadosFuncionais {
    pub fn new(salario: f64) -> Self {
        Self { salario }
    }

    pub fn salario(&self) -> f64 {
        self.salario
    }

    pub fn set_salario(&mut self, valor: f64) {
        self.salario = valor;
    }
}

#[derive(Debug)]
pub struct FuncionarioRegular {
    dados_pessoais: DadosPessoais,
    dados_funcionais: DadosFuncionais,
    beneficios: Beneficio,
}

impl FuncionarioRegular {
    pub fn new(
        dados_pessoais: DadosPessoais,
        dados_funcionais: DadosFuncionais,
        beneficios: Beneficio,
    ) -> Self {
        let mut result = Self {
            dados_pessoais,
            dados_funcionais,
            beneficios,
        };

        result.dados_funcionais.set_salario(
            result.dados_funcionais.salario() + result.beneficios.calcula_beneficios(),
        );

        result
    }

    pub fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    pub fn beneficios(&self) -> &Beneficio {
        &self.beneficios
    }

    pub fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }
}

impl Assalariado for FuncionarioRegular {
    fn dados_funcionais_mut(&mut self) -> &mut DadosFuncionais {
        &mut self.dados_funcionais
    }
}

impl Assalariado for Gerente {
    fn dados_funcionais_mut(&mut self) -> &mut DadosFuncionais {
        &mut self.dados_funcionais
    }
}

impl AtributosFuncionais for FuncionarioRegular {
    fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }
}

impl AtributosFuncionais for Gerente {
    fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }
}

#[derive(Debug)]
pub struct Gerente {
    dados_pessoais: DadosPessoais,
    dados_funcionais: DadosFuncionais,
    nro_vagas_garagem: u8,
}

impl Gerente {
    pub fn new(
        dados_pessoais: DadosPessoais,
        dados_funcionais: DadosFuncionais,
        nro_vagas_garagem: u8,
    ) -> Self {
        Self {
            dados_pessoais,
            dados_funcionais,
            nro_vagas_garagem,
        }
    }

    pub fn dados_pessoais(&self) -> &DadosPessoais {
        &self.dados_pessoais
    }

    pub fn dados_funcionais(&self) -> &DadosFuncionais {
        &self.dados_funcionais
    }

    pub fn nro_vagas_garagem(&self) -> u8 {
        self.nro_vagas_garagem
    }
}

pub trait Assalariado {
    fn dados_funcionais_mut(&mut self) -> &mut DadosFuncionais;
}

pub trait AtributosFuncionais: Assalariado {
    fn dados_funcionais(&self) -> &DadosFuncionais;
    fn dados_pessoais(&self) -> &DadosPessoais;
}

//Polimorfismo em Rust usando traits
pub fn desconta_falta<T: Assalariado>(pessoa: &mut T, valor: f64) {
    let mut dados_funcionais = pessoa.dados_funcionais_mut();
    dados_funcionais.set_salario(dados_funcionais.salario() - valor);
}
