const MAX: usize = 512;

struct Vectores {
    dimension: usize,
    elemento: [u64; MAX],
}

impl Vectores {
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elemento: [0; MAX],
        }
    }

    fn dim(&self) -> usize {
        self.dimension
    }

    fn dimensionar(&mut self, d: usize) {
        self.dimension = d;
    }

    fn addelemento(&mut self, e: u64) {
        if self.dimension < MAX {
            self.elemento[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn mostrar(&self) {
        for i in 0..self.dimension {
            println!("    elementos[{}] = {}", i, self.elemento[i]);
        }
    }

    fn buscar(&self, e: u64) -> usize {
        for i in 0..self.dimension {
            if self.elemento[i] == e {
                return i;
            }
        }
        return 100;
    }

    fn insertarele(&mut self, e: u64, p: usize) {
        if p > 0 && p <= self.dimension {
            self.dimension += 1;
            let mut inicial = self.dimension - 1;
            while inicial > p {
                self.elemento[inicial] = self.elemento[inicial - 1];
                inicial -= 1;
            }
            self.elemento[p - 1] = e;
        }
    }

    fn reemplazarele(&mut self, e: u64, p: usize) {
        if p > 0 && p <= self.dimension {
            self.elemento[p - 1] = e;
        }
    }

    fn obtener_pos(&self, e: u64) -> usize {
        for i in 0..self.dimension {
            if self.elemento[i] == e {
                return i + 1;
            }
        }
        0
    }

    fn obt_pos_real(&self, e: u64) -> usize {
        for i in 0..self.dimension {
            if self.elemento[i] == e {
                return i;
            }
        }
        600
    }

    fn invertir(&mut self) {
        let mut dimension = self.dimension - 1;
        let mut invertido = 0;

        while invertido < dimension {
            let aux = self.elemento[invertido];
            self.elemento[invertido] = self.elemento[dimension];
            self.elemento[dimension] = aux;

            invertido += 1;
            dimension -= 1;
        }
    }

    fn eliminar(&mut self, e: u64) {
        let pos = self.buscar(e);
        if pos != 100 {
            for i in pos..self.dimension {
                self.elemento[i] = self.elemento[i + 1];
            }
        }
        self.dimension -= 1;
    }

    fn ceros_atras(&mut self) {
        let mut pos = 0;

        for i in 0..self.dimension {
            if self.elemento[i] != 0 {
                self.elemento[pos] = self.elemento[i];
                pos += 1;
            }
        }
        while pos < self.dimension {
            self.elemento[pos] = 0;
            pos += 1;
        }
    }

    fn eliminar_numeros_n(&mut self, n: u64) {
        let mut pos: usize = 0;

        for i in 0..self.dimension {
            if self.elemento[i] > n {
                self.elemento[pos] = self.elemento[i];
                pos += 1;
            }
        }

        self.dimension = pos;
    }
}

fn main() {
    let mut v = Vectores::new();

    v.addelemento(7);
    v.addelemento(9);
    v.addelemento(3);
    v.addelemento(4);

    v.mostrar();
    println!("la dimension del vector es: {}", v.dim());

    println!("--------------------------------------");
    println!("los numeros menores < n fueron eliminados");
    v.eliminar_numeros_n(5);
    v.mostrar();

    println!("--------------------------------------");
    println!("el numero ya esta eliminado:");
    v.eliminar(3);
    v.mostrar();

    println!("--------------------------------------");
    println!("inserta el elemento 2, en la posicion 2: ");
    v.insertarele(2, 2);
    v.mostrar();

    let mut v2 = Vectores::new();
    v2.addelemento(7);
    v2.addelemento(9);
    v2.addelemento(3);
    v2.addelemento(4);

    println!("--------------------------------------");
    println!("el invertido es: ");
    v2.invertir();
    v2.mostrar();

    println!("--------------------------------------");
    println!("reemplaza el elemento 99, en la posicion 2: ");
    v2.reemplazarele(99, 2);
    v2.mostrar();

    let mut v3 = Vectores::new();

    v3.addelemento(1);
    v3.addelemento(0);
    v3.addelemento(3);
    v3.addelemento(0);
    v3.addelemento(2);
    v3.addelemento(2);

    println!("--------------------------------------");
    println!("funcion de los ceros atras:");
    v3.ceros_atras();
    v3.mostrar();
}
