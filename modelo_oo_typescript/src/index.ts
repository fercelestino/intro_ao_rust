class Beneficio {
    constructor(private valeAlimentacao: number, private fgts: number) {}

    getValeAlimentacao(): number {
        return this.valeAlimentacao;
    }

    getFgts(): number {
        return this.fgts;
    }

    calculaBeneficios(): number {
        return this.getValeAlimentacao() + this.getFgts();
    }
}

class DadosPessoais {
    constructor(private nome: string, private idade: number) {}

    getNome(): string {
        return this.nome;
    }

    getIdade(): number {
        return this.idade;
    }

    clone(): DadosPessoais {
        return new DadosPessoais(this.nome, this.idade);
    }
}

class DadosFuncionais {
    constructor(private salario: number) {}

    getSalario(): number {
        return this.salario;
    }

    setSalario(salario: number): void {
        this.salario = salario;
    }

    clone(): DadosFuncionais {
        return new DadosFuncionais(this.salario);
    }
}

class FuncionarioRegular implements Assalariado, AtributosFuncionais {
    constructor(
        private dadosPessoais: DadosPessoais,
        private dadosFuncionais: DadosFuncionais,
        private beneficios: Beneficio
    ) {
        this.dadosFuncionais.setSalario(
            this.dadosFuncionais.getSalario() +
                this.beneficios.calculaBeneficios()
        );
    }

    getDadosFuncionais(): DadosFuncionais {
        return this.dadosFuncionais;
    }

    getDadosPessoais(): DadosPessoais {
        return this.dadosPessoais;
    }

    getBeneficios(): Beneficio {
        return this.beneficios;
    }
}

class Gerente implements Assalariado, AtributosFuncionais {
    constructor(
        private dadosPessoais: DadosPessoais,
        private dadosFuncionais: DadosFuncionais,
        private nroVagasGaragem: number
    ) {}

    getDadosFuncionais(): DadosFuncionais {
        return this.dadosFuncionais;
    }

    getDadosPessoais(): DadosPessoais {
        return this.dadosPessoais;
    }

    getNroVagaGaragem(): number {
        return this.nroVagasGaragem;
    }
}

interface Assalariado {
    getDadosFuncionais(): DadosFuncionais;
}

interface AtributosFuncionais {
    getDadosPessoais(): DadosPessoais;
    getDadosFuncionais(): DadosFuncionais;
}

function daAumento(pessoa: Assalariado, aumento: number): void {
    const salarioAtual = pessoa.getDadosFuncionais().getSalario();
    pessoa.getDadosFuncionais().setSalario(salarioAtual + aumento);
}

function descontaFalta(pessoa: Assalariado, valor: number): void {
    const salarioAtual = pessoa.getDadosFuncionais().getSalario();
    pessoa.getDadosFuncionais().setSalario(salarioAtual - valor);
}

function promove(pessoa: FuncionarioRegular, nroVagaGaragem: number): Gerente {
    const result = new Gerente(
        pessoa.getDadosPessoais().clone(),
        pessoa.getDadosFuncionais().clone(),
        nroVagaGaragem
    );
    result
        .getDadosFuncionais()
        .setSalario(result.getDadosFuncionais().getSalario() * 2);

    return result;
}

function geraFicha(pessoa: AtributosFuncionais): string {
    let cracha = `Nome: ${pessoa.getDadosPessoais().getNome()}\nIdade: ${pessoa
        .getDadosPessoais()
        .getIdade()}\nSalário: ${pessoa.getDadosFuncionais().getSalario()}`;
    return cracha;
}

const funcionario1 = new FuncionarioRegular(
    new DadosPessoais("Fulano", 25),
    new DadosFuncionais(1000),
    new Beneficio(100, 100)
);
// const funcionario2 = new Pessoa("Sicrano", 25, 1000);
const funcionario2 = new FuncionarioRegular(
    new DadosPessoais("Sicrano", 25),
    new DadosFuncionais(1000),
    new Beneficio(100, 100)
);

let funcionarioDoMes = funcionario1;
let funcionariosFaltantes = [funcionario1, funcionario2];

console.log(funcionarioDoMes.getDadosPessoais().getNome());

daAumento(funcionarioDoMes, 200);

funcionariosFaltantes.forEach((funcionario) => {
    descontaFalta(funcionario, 50);
});

daAumento(funcionario1, 300);

console.log("Antes: " + funcionario1.getDadosFuncionais().getSalario());

let funcionario1Promovido = promove(funcionario1, 2);

console.log("Depois: " + funcionario1.getDadosFuncionais().getSalario());
console.log(
    "Depois Certo: " + funcionario1Promovido.getDadosFuncionais().getSalario()
);

// funcionario1.getDadosFuncionais().setSalario(0);
// console.log(
//     "Depois adulterado: " +
//         funcionario1Promovido.getDadosFuncionais().getSalario()
// );

console.log(geraFicha(funcionario1));
console.log(geraFicha(funcionario1Promovido));
console.log(funcionario1Promovido.getNroVagaGaragem());
