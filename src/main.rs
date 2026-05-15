fn main() {
    //PREGUNTA 1: ¿Qué ocurre con ownership cuando se usa Option::take()? ---
    //
    // Option::take() extrae el valor que está dentro del Option,
    // dejando ese Option como None. transfiere el ownership (propiedad)
    // del valor hacia quien lo recibe, sin necesidad de clonar.
    //
    // Por ejemplo en rotar_derecha():
    //   let mut x = y.izquierdo.take()
    //   y.izquierdo ahora es None
    //   x toma ownership del nodo que estaba ahí
    //
    // Es importante en las rotaciones porque necesitamos mover
    // nodos entre posiciones del árbol. Si no usámos take(),
    // Rust no permitiría tener dos referencias mutables al mismo
    // nodo al mismo tiempo.

    // --- PREGUNTA 2: ¿Por qué Box<Nodo> y no Nodo directamente? ---
    //
    // Un Nodo contiene dentro de sí mismo otros Nodos (izquierdo y derecho).
    // Rust no podría calcular cuánta memoria ocupa Nodo y fuese infinitamente.
    //
    // Box<Nodo> soluciona esto porque Box es un puntero al heap.
    // Su tamaño es fijo sin importar qué tan grande sea el Nodo al que apunta.
    // Así Rust puede calcular el tamaño en tiempo de compilación.
    // AVL AQUI INSERTO EL ARBOLITO CON LOS DATOS
    /*AVL

    DATOS: [5000, 3000, 2000, 4000, 3500, 6000]

    1:         5000

            (árbol balanceado, balance=0)

    2:  	 3000

            5000
           /
         3000
         (balance=1)

    3: 	2000

            5000
           /
         3000
         /
       2000
       (balance=2 en 5000 - Rotación simple derecha)

            3000
           /    \
         2000   5000
         4: 	 4000
                3000
               /    \
             2000   5000
                    /
                  4000
                  (balance=-1 en 3000)

        5: 	3500

                3000
               /    \
             2000   5000
                    /
                  4000
                  /
                3500
                (balance=-2 en 3000 - Rotación Doble, Derecha-Izquierda)
                Primero rotación derecha en 5000, luego izquierda en 3000 y nos queda
           Resultado:
                  3500
                 /    \
               3000   5000
               /      /
             2000   4000
             6: 	 6000

                       3500
                      /    \
                    3000   5000
                    /      /  \
                  2000   4000  6000
                  (balance=0, árbol balanceado)

             Rotaciones:
             3 → Rotación SIMPLE DERECHA (Izquierda-Izquierda)
             5 → Rotación DOBLE Derecha-Izquierda (Derecha-Izquierda)*/

    #[derive(Debug, Clone)]
    struct Vuelo {
        id: String,
        altitud: u32, // Este será nuestra clave (key)
    }

    struct Nodo {
        vuelo: Vuelo,
        izquierdo: Option<Box<Nodo>>,
        derecho: Option<Box<Nodo>>,
        altura: i32,
    }

    impl Nodo {
        fn nuevo(vuelo: Vuelo) -> Self {
            Nodo {
                vuelo,
                izquierdo: None,
                derecho: None,
                altura: 1,
            }
        }
    }

    // --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

    fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
        nodo.as_ref().map_or(0, |n| n.altura)
    }

    fn actualizar_altura(nodo: &mut Nodo) {
        nodo.altura = 1 + std::cmp::max(
            obtener_altura(&nodo.izquierdo),
            obtener_altura(&nodo.derecho),
        );
    }
    fn obtener_balance(nodo: &Nodo) -> i32 {
        obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
    }

    fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
        let mut x = y.izquierdo.take().expect("Error de radar");
        y.izquierdo = x.derecho.take();
        actualizar_altura(&mut y);
        x.derecho = Some(y);
        actualizar_altura(&mut x);
        x
    }

    fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
        let mut y = x.derecho.take().expect("Error de radar");
        x.derecho = y.izquierdo.take();
        actualizar_altura(&mut x);
        y.izquierdo = Some(x);
        actualizar_altura(&mut y);
        y
    }

    // --- FUNCIÓN DE INSERCIÓN ---

    fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
        let mut nodo = match nodo_opt {
            None => return Box::new(Nodo::nuevo(vuelo)),
            Some(n) => n,
        };

        if vuelo.altitud < nodo.vuelo.altitud {
            nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo));
        } else if vuelo.altitud > nodo.vuelo.altitud {
            nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo));
        } else {
            return nodo;
        }

        actualizar_altura(&mut nodo);
        let balance = obtener_balance(&nodo);

        // Caso Izquierda-Izquierda
        if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
            return rotar_derecha(nodo);
        }
        // Caso Derecha-Derecha
        if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
            return rotar_izquierda(nodo);
        }
        // Caso Izquierda-Derecha
        if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
            let hijo_izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
            return rotar_derecha(nodo);
        }
        // Caso Derecha-Izquierda
        if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
            let hijo_der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(rotar_derecha(hijo_der));
            return rotar_izquierda(nodo);
        }

        nodo
    }

    fn main() {
        let mut radar: Option<Box<Nodo>> = None;

        // Simulación de entrada de vuelos
        let datos = vec![
            ("AV123", 5000),
            ("UA456", 3000),
            ("IB101", 2000),
            ("AF999", 4000),
            ("TA222", 3500),
            ("AM777", 6000),
        ];

        for (id, alt) in datos {
            let v = Vuelo {
                id: id.to_string(),
                altitud: alt,
            };
            radar = Some(insertar(radar.take(), v));
        }

        println!("--- Radar de Control Aéreo (AVL) ---");
        // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación
    }
}
